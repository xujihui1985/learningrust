use serde::{de::Error, Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, RwLock},
};
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use warp::{self, path::Exact};
use warp::{http, Filter};

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone)]
struct Store {
    grocery_list: Arc<RwLock<Items>>,
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_store(store: Store) -> impl Filter<Extract = (Store,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let p = warp::path("v1")
        .and(warp::path("hello"))
        .and(warp::path::end());
    let add_items = warp::post()
        .and(p)
        .and(json_body())
        .and(with_store(store))
        .and_then(add_grocery_list_item);
    warp::serve(add_items).run(([127, 0, 0, 1], 3000)).await
}

async fn add_grocery_list_item(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .grocery_list
        .write()
        .unwrap()
        .insert(item.name, item.quantity);
    Ok(warp::reply::with_status(
        "add success",
        http::StatusCode::CREATED,
    ))
}
