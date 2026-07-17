use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Central error type for all API handlers.
/// Every handler returns `Result<T, AppError>` instead of panicking.
#[derive(Debug)]
pub enum AppError {
    /// Any database failure (connection, query, constraint, etc.)
    Database(sqlx::Error),
    /// OpenAI request failed or returned something unusable
    ImageGeneration(String),
    /// Requested resource (e.g. a job id) doesn't exist
    NotFound(String),
    /// Bad credentials, missing/invalid token, etc.
    Auth(String),
    /// e.g. signup with an email that's already taken
    Conflict(String),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                eprintln!("database error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            AppError::ImageGeneration(msg) => {
                eprintln!("image generation error: {msg}");
                (StatusCode::BAD_GATEWAY, "image generation failed".to_string())
            }
            AppError::NotFound(what) => (StatusCode::NOT_FOUND, format!("{what} not found")),
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        };

        (status, Json(ErrorBody { error: message })).into_response()
    }
}

// Lets `?` auto-convert sqlx::Error into AppError inside handlers.
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e)
    }
}