use std::net::TcpListener;

use sqlx::PgPool;
use zerotoproduct::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "zero2prod".into(), 
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let config = get_configuration().expect("failed to read configuration");
    let conn = PgPool::connect(&config.database.connection_string())
        .await
        .expect("failed to connect to database");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&address)?;
    run(listener, conn)?.await
}
