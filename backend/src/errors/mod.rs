mod auth;

use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug)]
pub struct AppError {
    pub code: &'static str,     // business error code
    pub message: String,        // what you want the client to see
    pub status: StatusCode,     // HTTP status
    pub details: Option<Value>, // optional details
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let body = Json(ErrorResponse {
            code: self.code.to_string(),
            message: self.message,
            details: self.details,
        });

        (self.status, body).into_response()
    }
}

impl AppError {
    pub fn new(code: &'static str, message: impl Into<String>, status: StatusCode) -> Self {
        Self {
            code,
            message: message.into(),
            status,
            details: None,
        }
    }

    pub fn with_details<T>(
        code: &'static str,
        message: impl Into<String>,
        status: StatusCode,
        details: T,
    ) -> Self
    where
        T: Serialize,
    {
        let details = serde_json::to_value(details).unwrap_or_else(|_| Value::Null);

        Self {
            code,
            message: message.into(),
            status,
            details: Some(details),
        }
    }
}
