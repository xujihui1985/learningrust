use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer, middleware::Logger};
use tracing_actix_web::TracingLogger;
use sqlx::{PgPool};

pub fn run(listener: TcpListener, pool: PgPool) -> std::io::Result<Server> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
