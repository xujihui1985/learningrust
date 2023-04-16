use std::time::{SystemTime, UNIX_EPOCH};

use crate::{ctx::Ctx, error::ClientError, prelude::*};
use axum::http::Uri;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,
    req_path: String,
    req_method: String,
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<String>,
}

pub async fn log_request(
    uuid: String,
    req_method: String,
    uri: Uri,
    ctx: Option<Ctx>,
    server_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let error_type = server_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(server_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        ..
    };
    todo!()
}
