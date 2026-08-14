use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::validation::ValidationError;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Hashing the password failed: {0}")]
    Hash(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid user id")]
    InvalidId,
    #[error("Invite has already been used")]
    InviteUsed,
    #[error("Username has been taken")]
    UsernameTaken,
    #[error("Passwords do not match")]
    PasswordMismatch,
}

impl From<argon2::password_hash::Error> for UserError {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::Hash(e.to_string())
    }
}

#[derive(Debug, Error)]
pub enum PageError {
    #[error("Slug has been taken")]
    SlugTaken,
    #[error("No page with this slug exists")]
    NotFound,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] UserError),
    #[error(transparent)]
    Page(#[from] PageError),
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[allow(clippy::enum_glob_use)]
        use AppError::*;
        match &self {
            Auth(UserError::InvalidCredentials) => (
                StatusCode::UNAUTHORIZED,
                format!("{}", UserError::InvalidCredentials),
            )
                .into_response(),
            Auth(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response(),
            Page(PageError::NotFound) => {
                (StatusCode::NOT_FOUND, format!("{}", PageError::NotFound)).into_response()
            },
            Page(err) => (StatusCode::BAD_REQUEST, format!("{err}")).into_response(),
            Validation(err) => (StatusCode::BAD_REQUEST, format!("{err}")).into_response(),
            Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
        }
    }
}
