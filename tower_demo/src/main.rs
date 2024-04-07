#![allow(warnings)]
use std::convert::Infallible;
use std::fmt::{Display, Error, Formatter};
// infallible means never fail
use futures::future::{ready, BoxFuture, Ready};
use futures::{AsyncWriteExt, FutureExt};
use hyper::body::HttpBody;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tower::{BoxError, Layer, Service, ServiceBuilder};

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("hello world")))
}

#[derive(Clone, Copy)]
struct HelloWorld;

impl Display for HelloWorld {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), Error> { 

        todo!() 
    }

}

//impl Service<Request<Body>> for HelloWorld {
//type Response = Response<Body>;
//type Error = Infallible;
//    type Future = Ready<Result<Self::Response, Self::Error>>;
//type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

//fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
//std::task::Poll::Ready(Ok(()))
//}
//fn call(&mut self, req: Request<Body>) -> Self::Future {
//Box::pin(async move {
//log::debug!("recv req {} {}", req.method(), req.uri().path());
//let response = ready(Ok(Response::new(Body::from("hello world")))).await;
//log::debug!("after process");
//response
//})
//}
//}

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        Box::pin(async move {
            log::info!("before");
            let resp = ready(Ok(Response::new(Body::from("hello world")))).await;
            log::info!("after");
            resp
        })
    }
}

// middleware
#[derive(Clone, Copy)]
struct Logging<S> {
    inner: S,
}

impl<S> Logging<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for Logging<S>
where
    S: Service<Request<B>> + Clone + Send + 'static,
    B: HttpBody + Send + 'static,
    S::Future: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    //type Future = S::Future;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            let method = req.method().clone();
            let path = req.uri().path().to_string();
            log::debug!("process req {} {}", method, path);
            let response = inner.call(req).await;
            log::debug!("finish process req {} {}", method, path);
            response
        })
    }
}

#[derive(Clone, Copy)]
struct LoggingNoBox<S> {
    inner: S,
}

impl<S> LoggingNoBox<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

// we don't want to have heap allocation on every request with Box pin
impl<S, B> Service<Request<B>> for LoggingNoBox<S>
where
    S: Service<Request<B>> + Clone + Send + 'static,
    B: Send + 'static,
    S::Future: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LoggingFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        log::debug!("before process request {} {}", method, path);
        let start = Instant::now();
        LoggingFuture {
            inner: self.inner.call(req),
            method,
            path,
            start,
        }
    }
}

struct LoggingFuture<F> {
    inner: F,
    method: hyper::Method,
    path: String,
    start: std::time::Instant,
}

impl<F> Future for LoggingFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = unsafe {
            let this = self.get_unchecked_mut();
            let res = Pin::new_unchecked(&mut this.inner).poll(cx);
            let duration = this.start.elapsed();
            log::debug!(
                "finish process request {} {} time={:?}",
                this.method,
                this.path,
                duration
            );
            res
        };
        result
    }
}

#[derive(Clone, Copy)]
struct Timeout<S> {
    inner: S,
    timeout: std::time::Duration,
}

impl<S> Timeout<S> {
    fn new(inner: S, timeout: std::time::Duration) -> Self {
        Self { inner, timeout }
    }
}

// Timeout don't care about the type of request, any request can be arguments of
// service
impl<S, R> Service<R> for Timeout<S>
where
    S: Service<R>,
    //S::Error: std::error::Error + Send + Sync + 'static,
    S::Error: Into<BoxError> + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = tower::BoxError; // when timeout happened we need to response timeout error otherwise we need to return inner error
    type Future = TimeoutFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        //match self.inner.poll_ready(cx) {
        //Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
        //Poll::Ready(Err(err)) => Poll::Ready(Err(Box::new(err))),
        //Poll::Pending => Poll::Pending,
        //}
        //self.inner.poll_ready(cx).map(|result| result.map_err(|err| Box::new(err) as tower::BoxError))
        //self.inner.poll_ready(cx).map_err(|err| Box::new(err) as tower::BoxError)
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: R) -> Self::Future {
        TimeoutFuture {
            inner: self.inner.call(req),
            sleep: tokio::time::sleep(self.timeout),
        }
    }
}

struct TimeoutFuture<F> {
    inner: F,
    sleep: tokio::time::Sleep,
}

impl<F, T, E> Future for TimeoutFuture<F>
where
    F: Future<Output = Result<T, E>>,
    //E: std::error::Error + Send + Sync + 'static,
    E: Into<BoxError> + Send + Sync + 'static,
{
    type Output = Result<T, tower::BoxError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            let this = self.get_unchecked_mut();
            match Pin::new_unchecked(&mut this.inner).poll(cx) {
                Poll::Pending => {}
                Poll::Ready(result) => {
                    return match result {
                        Ok(res) => Poll::Ready(Ok(res)),
                        Err(err) => Poll::Ready(Err(err.into())),
                    }
                }
            };

            match Pin::new_unchecked(&mut this.sleep).poll(cx) {
                Poll::Pending => {}
                Poll::Ready(_) => return Poll::Ready(Err(Elapsed.into())),
            }
        };
        Poll::Pending
    }
}

#[derive(Debug)]
struct Elapsed;

impl Display for Elapsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "timeout elapsed")
    }
}

impl std::error::Error for Elapsed {}

struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = Logging<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Logging::new(inner)
    }
}

struct TimeoutLayer {
    timeout: Duration,
}

impl TimeoutLayer {
    fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = Timeout<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Timeout::new(inner, self.timeout)
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_service = make_service_fn(|_conn| {
        let c = _conn;
        let a = async {
            //let svc = HelloWorld;
            //let svc = TimeoutLayer::new(Duration::from_secs(20)).layer(svc);
            //let svc = LoggingNoBox::new(svc);
            //let s = ServiceBuilder::new()
            //.layer(TimeoutLayer::new(Duration::from_secs(20)))
            //.layer(LoggingLayer)
            //.service(svc);
            //let c = _conn;

            Ok::<_, Infallible>(Logging::new(HelloWorld))
        };
        a
    });
    let server = Server::bind(&addr).serve(make_service);
    if let Err(e) = server.await {
        println!("server err {}", e);
    }
}
