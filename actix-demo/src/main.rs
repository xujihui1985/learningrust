use std::sync::mpsc;
use std::thread;
use futures::future::Future;

use actix_web::{middleware, App, HttpResponse, HttpServer, Responder, web};

fn index() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index").to(|| "world"))
            .service(web::resource("/").to(index)
        )
    })
        .bind("127.0.0.1:8000")?
        .run()

}
