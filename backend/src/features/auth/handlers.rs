use super::schemas::RegisterRequest;
use super::service;
use crate::error::AppError;
use crate::state::SharedState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn register(
    State(state): State<SharedState>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = service::register_user(&state.db, &body).await?;
    let session = service::new_session(&state.db, user_id).await?;

    Ok((StatusCode::CREATED, Json("User registered successfully")))
}

pub async fn login(State(_): State<SharedState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn logout(State(_): State<SharedState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn logout_everywhere(State(_): State<SharedState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
