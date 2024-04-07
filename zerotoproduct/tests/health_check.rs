use std::net::TcpListener;

use once_cell::sync::Lazy;
use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zerotoproduct::{
    self,
    configuration::{get_configuration, DatabaseSettings}, telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);
});

struct TestContext {
    addr: String,
    pool: PgPool,
}

async fn spawn_app() -> TestContext {
    Lazy::force(&TRACING);
    // random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut cfg = get_configuration().expect("failed to read config");
    cfg.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_database(&cfg.database).await;
    // let pool = PgPool::connect(&cfg.database.connection_string())
    //     .await
    //     .expect("failed to connect to db");
    let server = zerotoproduct::startup::run(listener, pool.clone()).expect("failed to run server");
    let _ = tokio::spawn(server);
    TestContext {
        addr: address,
        pool: pool,
    }
}

async fn configure_database(cfg: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect(&cfg.connection_string_without_db())
        .await
        .expect("failed to connect db");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, cfg.database_name).as_str())
        .await
        .expect("failed to create database");
    let conn_pool = PgPool::connect(&cfg.connection_string())
        .await
        .expect("failed to connect to db");
    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("failed to migrate the db");
    conn_pool
}

#[tokio::test]
async fn health_check_works() {
    let ctx = spawn_app().await;

    let client = Client::new();
    let resp = client
        .get(format!("{}/health_check", &ctx.addr))
        .send()
        .await
        .expect("failed to execute request");
    assert!(resp.status().is_success());
}

#[tokio::test]
async fn subscribe_return_200_for_valid_form_data() {
    let ctx = spawn_app().await;
    let client = Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let resp = client
        .post(&format!("{}/subscriptions", &ctx.addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");
    assert_eq!(200, resp.status().as_u16());
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
    let ctx = spawn_app().await;
    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=le%20guin", "missing the name"),
        ("", "missing both email and name"),
    ];
    for (invalid_body, message) in test_cases {
        let resp = client
            .post(&format!("{}/subscriptions", &ctx.addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed execute request");
        assert_eq!(400, resp.status().as_u16(), "failed with 400 {}", message);
    }
}
