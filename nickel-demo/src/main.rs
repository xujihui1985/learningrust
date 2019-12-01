use nickel::Nickel;
use nickel::{router, _router_inner, middleware, _middleware_inner, as_pat, as_block};
use std::io;
use std::io::prelude::*;
use std::fs::{OpenOptions};

fn say_hello() -> &'static str {
    "hello"
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    server.listen("127.0.0.1:8080").unwrap();
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut f = OpenOptions::new()
                .append(true)
                .write(true)
                .create(true)
                .open(filename)
                .unwrap();

    f.write_all(bytes)
}
