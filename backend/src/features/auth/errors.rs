use crate::error::AppError;
use axum::http::StatusCode;

#[derive(Debug)]
pub enum AuthError {
    UserAlreadyExists(String), // user identifier
    InvalidCredentials,
    MissingToken,
    ExpiredToken,
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::UserAlreadyExists(identifier) => AppError::new(
                "USER_ALREADY_EXISTS",
                format!("User {identifier} already exists"),
                StatusCode::CONFLICT,
            ),
            AuthError::InvalidCredentials => AppError::new(
                "INVALID_CREDENTIALS",
                "Invalid identifier or password",
                StatusCode::UNAUTHORIZED,
            ),
            AuthError::MissingToken => AppError::new(
                "MISSING_SESSION_TOKEN",
                "Session token is missing. Please log in.",
                StatusCode::UNAUTHORIZED,
            ),
            AuthError::ExpiredToken => AppError::new(
                "EXPIRED_SESSION_TOKEN",
                "Session token is expired. Please log in again.",
                StatusCode::UNAUTHORIZED,
            ),
        }
    }
}
