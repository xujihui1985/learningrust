use crate::log::log_request;
use crate::prelude::{Error, Result as AxumError};
#[allow(unused)]
use axum::extract::Path;
use axum::extract::Query;
use axum::handler::Handler;
use axum::http::{HeaderMap, Request, StatusCode, Uri, Method};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get_service;
use axum::{http, middleware, routing::get, Extension, Json, Router};
use ctx::Ctx;
use model::ModelController;
use serde::Deserialize;
use serde_json::{json, Value};
#[allow(unused)]
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use web::mw_auth::{mw_ctx_resolver, mw_require_auth};
use web::{routes_login, routes_tickets};

mod ctx;
mod error;
mod log;
mod model;
mod prelude;
mod web;

struct State {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mc = ModelController::new()?;

    let routes_apis =
        routes_tickets::routes(mc.clone()).route_layer(middleware::from_fn(mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis) //nest under /api
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(mc.clone(), mw_ctx_resolver))
        .layer(CookieManagerLayer::new()) // this run first, from bottom to top
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await?;
    Ok(())

    // tracing_subscriber::fmt::init();

    // let shared_state = Arc::new(State { name: String::from("hello") });
    // let layerd_handler = shared.layer(Extension(shared_state));// .layer(middleware::from_fn(my_middleware));
    // let app = Router::new()
    //     .route("/foo/:user_id", get(foo))
    //     .route("/json", get(json))
    //     .route("/shared", get(layerd_handler));
    //     // .layer(middleware::from_fn(my_middleware))
    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // tracing::debug!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

async fn main_response_mapper(ctx: Option<Ctx>, uri: Uri, req_method: Method, res: Response) -> Response {
    println!("->> {:<12} main response", "RES MAPPER");
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": "testid",
                }
            });
            (*status_code, Json(client_error_body)).into_response()
        });
    let client_error = client_status_error.unzip().1;
    // log_request(uuid, req_method, uri, ctx, server_error, client_error)
    error_response.unwrap_or(res)
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let a = String::from("hello");
    // 这里用deref将String deref 到 &str是为了不用alloc，如果这里用as_ref, 那么当unwrap_or(<default>), 我们就需要一个&String
    let name = params.name.as_deref().unwrap_or("World");
    // let name = params.name.as_ref().unwrap_or(&String::from("World"));
    Html("<H1> hello {name} </H1>")
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("<H1> hello {name} </H1>"))
}

async fn my_middleware<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    match header {
        Some(header) => Ok(next.run(req).await),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn foo(Path(user_id): Path<u32>, headers: HeaderMap) -> &'static str {
    println!("userid {}", user_id);
    println!("headers {:?}", headers);
    "hello"
}

async fn json() -> impl IntoResponse {
    (StatusCode::CREATED, Json(json!({"data": 123})))
}

async fn shared(Extension(state): Extension<Arc<State>>) -> Json<Value> {
    Json(json!({"data": state.name}))
}
