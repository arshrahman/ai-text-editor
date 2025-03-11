use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Environment(String),
    Server(String),
    GeminiApi(String),
    Serialization(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Environment(msg) => write!(f, "Environment error: {}", msg),
            AppError::Server(msg) => write!(f, "Server error: {}", msg),
            AppError::GeminiApi(msg) => write!(f, "Gemini API error: {}", msg),
            AppError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Environment(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Server(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::GeminiApi(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::Serialization(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({
            "error": {
                "message": message,
                "code": status.as_u16()
            }
        }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;