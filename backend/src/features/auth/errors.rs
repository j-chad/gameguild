use crate::error::AppError;
use axum::http::StatusCode;

#[derive(Debug)]
pub enum AuthError {
    UserAlreadyExists(String), // user identifier
    PasswordHashingFailed,
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::UserAlreadyExists(identifier) => AppError::new(
                "USER_ALREADY_EXISTS",
                format!("User {identifier} already exists"),
                StatusCode::CONFLICT,
            ),
            AuthError::PasswordHashingFailed => AppError::new(
                "PASSWORD_HASHING_FAILED",
                "Failed to hash password",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
