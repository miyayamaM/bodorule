use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InvalidRequest(#[from] ParseError),
    #[error("{0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("{0}")]
    ConversionEntityError(String),
    #[error("{0}")]
    EntityNotFoundError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::InvalidRequest(_) | AppError::EntityNotFoundError(_) => {
                StatusCode::BAD_REQUEST
            }
            AppError::ConversionEntityError(_) | AppError::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let error_message = ErrorResponse {
            message: self.to_string(),
        };
        (status_code, Json(error_message)).into_response()
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}
