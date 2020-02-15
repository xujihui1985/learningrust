use std::io::Error as IOError;

use reqwest::Error as ReqwestError;

use http::header::InvalidHeaderName as HttpInvalidHeaderNameError;
use http::header::InvalidHeaderValue as HttpInvalidHeaderValueError;

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    Reqwest(ReqwestError),
    Http(HttpError),
    App,
}

#[derive(Debug)]
pub enum HttpError {
    HttpInvalidHeaderName(HttpInvalidHeaderNameError),
    HttpInvalidHeaderValue(HttpInvalidHeaderValueError),
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Error::Reqwest(e)
    }
}

impl From<HttpInvalidHeaderValueError> for Error {
    fn from(e: HttpInvalidHeaderValueError) -> Self {
        Error::Http(HttpError::HttpInvalidHeaderValue(e))
    }
}
