use super::schemas::{LoginRequest, RegisterRequest, RegisterResponse};
use super::service;
use crate::error::AppError;
use crate::state::SharedState;
use axum::extract::{ConnectInfo, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use std::net::SocketAddr;
use validator::Validate;

pub async fn register(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;

    let ip_addr = addr.ip();
    let user_agent = headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(std::string::ToString::to_string);

    let user_id = service::register_user(&state.db, &body).await?;
    let session_token = service::new_session(&state.db, user_id, Some(ip_addr), user_agent).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            user_id: user_id.to_string(),
            session_token,
        }),
    ))
}

pub async fn login(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let ip_addr = addr.ip();
    let user_agent = headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(std::string::ToString::to_string);
}

pub async fn logout(State(_): State<SharedState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn logout_everywhere(State(_): State<SharedState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
