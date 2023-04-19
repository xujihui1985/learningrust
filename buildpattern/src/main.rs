use std::marker::PhantomData;

#[derive(Debug)]
pub struct Request {
    url: String,
    method: String,
    headers: Vec<(String,String)>,
    body: Option<String>,
}

// region: States
#[derive(Default, Clone)]
pub struct NoUrl;
#[derive(Default, Clone)]
pub struct Url(String);

#[derive(Default, Clone)]
pub struct NoMethod;
#[derive(Default, Clone)]
pub struct Method(String);


#[derive(Default, Clone)]
pub struct Sealed;
#[derive(Default, Clone)]
pub struct NotSealed(String);
// endregion States

#[derive(Default, Clone)]
pub struct RequestBuilder<U, M, S> {
    url: U,
    method: M,
    headers: Vec<(String,String)>,
    body: Option<String>,
    marker_seal: PhantomData<S>,
}

impl RequestBuilder<NoUrl, NoMethod, NotSealed> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

impl<S> RequestBuilder<Url, Method, S> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method.0,
            body: self.body,
            headers: self.headers,
        }
    }
}

impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn seal(self) -> RequestBuilder<U, M, Sealed> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }

}

impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, M, NotSealed> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }

    pub fn method(self, method: impl Into<String>) -> RequestBuilder<U, Method, NotSealed> {
        RequestBuilder {
            url: self.url,
            method: Method(method.into()),
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }

    pub fn headers(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }
}

fn main() {
    println!("Hello, world!");
    let req = RequestBuilder::new().url("aa").method("aaaa").build();
}
