use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InvalidRequest(#[from] ParseError),
    #[error("{0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("{0}")]
    ConversionEntityError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ConversionEntityError(_) | AppError::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status_code.into_response()
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}
