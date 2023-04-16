use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Clone, strum_macros::AsRefStr, Serialize)]
#[serde(tag="type", content="data")]
pub enum Error {
    #[error("failed to login")]
    LoginFail,

    #[error("failed to delete ticket {id}")]
    TicketDeleteIdNotFound { id: u64 },
    #[error("failed to auth as no auth token found")]
    AuthFailNoAuthToken,
    #[error("failed to auth as wrong token format")]
    AuthFailTokenWrongFormat,

    #[error("failed to auth ctx not in request")]
    AuthFailCtxNotInRequest,
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("error response");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
        // (StatusCode::INTERNAL_SERVER_ERROR, "unhandled client error").into_response()
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::AuthFailNoAuthToken => {
                (
                    StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    ClientError::NO_AUTH,
                )
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            )
        }
    }
}
