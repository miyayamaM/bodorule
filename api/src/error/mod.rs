use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InvalidRequest(#[from] ParseError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InvalidRequest(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}
