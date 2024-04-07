use std::{net::SocketAddr, convert::Infallible};

use hyper::{Server, Request, Body, Response, service::{make_service_fn, service_fn}, Client};


#[tokio::main]
async fn main() {
    let s = Some(String::from("hello"));
    let b = s.as_deref();
    let c = s.as_ref();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // make_service_fn is a closure that takes a generic argument and returns a service
    // it is a factory that generates services
    let make_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(handle))
    });
    // let make_svc = Shared::new(service_fn(handle));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
   let client = Client::new();
    client.request(req).await
}

