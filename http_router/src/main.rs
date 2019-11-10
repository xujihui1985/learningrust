use std::collections::HashMap;
use std::cell::RefCell;

struct Request {
    method: String,
    url: String,
    headers: RefCell<HashMap<String, String>>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter{
    routes: HashMap<String, BoxedCallback>
}

impl BasicRouter {
    pub fn new() -> Self {
        BasicRouter { routes: HashMap::new() }
    }

    fn add_route<C>(&mut self, url: &str, callback: C) where C: Fn(&Request) -> Response + 'static {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => Response{
                code: 404,
                body: Vec::new(),
                headers: HashMap::new(),
            },
            Some(callback) => callback(request),
        }
    }
}


fn main() {
    let mut route = BasicRouter::new();
    route.add_route("/", |_| Response { code: 200, body: Vec::new(), headers: HashMap::new() });
    route.add_route("/bb", |request| {
        let mut h = request.headers.borrow_mut();
        h.insert("hello".to_string(), "world".to_string());
        Response {
            headers: HashMap::new(),
            code: 201,
            body: Vec::new(),
        }
    });

    let req = Request{
        url: String::from("/"),
        method: String::from("GET"),
        headers: RefCell::new(HashMap::new()),
        body: Vec::new(),
    };
    let resp = route.handle_request(&req);
    println!("status code {}", resp.code);
}
