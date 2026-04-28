use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputexError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Security error: {0}")]
    SecurityError(String),

    #[error("Cryptography error: {0}")]
    CryptoError(String),

    #[error("Market error: {0}")]
    MarketError(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: String,
}

impl IntoResponse for ComputexError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            ComputexError::DatabaseError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "DB_ERROR", msg)
            }
            ComputexError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg),
            ComputexError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg),
            ComputexError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, "INVALID_REQUEST", msg),
            ComputexError::SecurityError(msg) => {
                (StatusCode::FORBIDDEN, "SECURITY_ERROR", msg)
            }
            ComputexError::CryptoError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "CRYPTO_ERROR", msg)
            }
            ComputexError::MarketError(msg) => {
                (StatusCode::BAD_REQUEST, "MARKET_ERROR", msg)
            }
            ComputexError::ProviderError(msg) => (StatusCode::BAD_REQUEST, "PROVIDER_ERROR", msg),
            ComputexError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg)
            }
        };

        let response = ErrorResponse {
            error: error_code.to_string(),
            message,
            code: error_code.to_string(),
        };

        (status, Json(response)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ComputexError>;
