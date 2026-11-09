use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(sqlx::Error),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal server error")]
    Internal,
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        if let sqlx::Error::Database(database_error) = &error {
            match database_error.code().as_deref() {
                Some("23505") => {
                    return AppError::Conflict(
                        "a resource with these values already exists".into(),
                    );
                }
                Some("23503") => {
                    return AppError::BadRequest("a referenced resource does not exist".into());
                }
                _ => {}
            }
        }

        tracing::error!(%error, "database operation failed");
        AppError::Database(error)
    }
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "database_error",
            AppError::BadRequest(_) => "bad_request",
            AppError::Unauthorized => "unauthorized",
            AppError::Forbidden => "forbidden",
            AppError::NotFound(_) => "not_found",
            AppError::Conflict(_) => "conflict",
            AppError::Internal => "internal_error",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = self.code();
        let (status, message) = match &self {
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database error".to_string(),
            ),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden".to_string()),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message.clone()),
            AppError::Conflict(message) => (StatusCode::CONFLICT, message.clone()),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}
