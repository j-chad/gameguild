use super::super::models;
use crate::error::AppError;
use crate::features::auth::errors::AuthError;
use crate::features::auth::middleware::session_token_extractor::SessionToken;
use crate::features::auth::queries;
use crate::state::SharedState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Debug)]
pub struct Session(pub models::Session);

impl FromRequestParts<SharedState> for Session {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        // Get token
        let token = SessionToken::from_request_parts(parts, state).await?;

        // Get session from token
        queries::get_session_by_token(&state.db, &token.0)
            .await?
            .map(Self)
            .ok_or_else(|| AppError::from(AuthError::ExpiredToken))
    }
}
