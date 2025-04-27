use super::super::errors::AuthError;
use crate::error::AppError;
use crate::state::SharedState;
use axum::extract::{FromRequest, FromRequestParts};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;

#[derive(Debug)]
pub struct SessionToken(pub String);

impl FromRequestParts<SharedState> for SessionToken {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Try cookie first
        let cookies = CookieJar::from_headers(&parts.headers);
        if let Some(cookie) = cookies.get(&state.config.auth.session_cookie_name) {
            return Ok(SessionToken(cookie.value().to_string()));
        }

        // 2. Try Authorization header
        if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                let token = auth_str.trim_start_matches("Bearer ");
                return Ok(SessionToken(token.to_string()));
            }
        }

        // 3. If neither is found, return an error
        Err(AuthError::MissingToken.into())
    }
}
