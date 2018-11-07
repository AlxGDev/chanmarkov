use serde_json;
use hyper;
use actix_web;


// Define a type so we can return multiple types of errors
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
    Other(String),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
} 

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

impl From<actix_web::client::SendRequestError> for FetchError {
    fn from(err: actix_web::client::SendRequestError) -> FetchError {
        FetchError::Other(err.to_string())
    }
}

impl From<actix_web::error::PayloadError> for FetchError {
    fn from(err: actix_web::error::PayloadError) -> FetchError {
        FetchError::Other(err.to_string())
    }
}