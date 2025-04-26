use crate::state::SharedState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn register(State(_): State<SharedState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
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
