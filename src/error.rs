use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::validation::ValidationError;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("password hashing failed: {0}")]
    Hash(String),
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("invite has been used")]
    InviteUsed,
    #[error("username has been taken")]
    UsernameTaken,
    #[error("passwords do not match")]
    PasswordMismatch,
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::Hash(e.to_string())
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
    #[error("bad request: {0}")]
    BadRequest(String),
}

#[allow(clippy::use_self)]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::BadRequest(_) | AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Auth(AuthError::InvalidCredentials) => StatusCode::UNAUTHORIZED,
            AppError::Auth(_) | AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            eprintln!("internal error: {self:?}");
        }
        (status, status.canonical_reason().unwrap_or("error")).into_response()
    }
}
