use super::schemas::{LoginRequest, NewSessionResponse, RegisterRequest};
use super::service;
use crate::error::AppError;
use crate::features::auth::middleware::Session;
use crate::state::SharedState;
use axum::extract::{ConnectInfo, State};
use axum::http::header::USER_AGENT;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use std::net::SocketAddr;
use validator::Validate;

pub async fn register(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    cookies: CookieJar,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;

    let user_id = service::register_user(&state.db, &body).await?;

    let ip_addr = addr.ip();
    let user_agent = headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(std::string::ToString::to_string);

    let session_token = service::new_session(&state.db, user_id, Some(ip_addr), user_agent).await?;

    Ok((
        StatusCode::CREATED,
        cookies.add(Cookie::new(
            state.config.auth.session_cookie_name.clone(),
            session_token.clone(),
        )),
        Json(NewSessionResponse {
            user_id: user_id.to_string(),
            session_token,
        }),
    ))
}

pub async fn login(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    cookies: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;

    let user_id = service::login_user(
        &state.db,
        body.username.as_deref(),
        body.email.as_deref(),
        &body.password,
    )
    .await?;

    let ip_addr = addr.ip();
    let user_agent = headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(std::string::ToString::to_string);

    let session_token = service::new_session(&state.db, user_id, Some(ip_addr), user_agent).await?;

    Ok((
        StatusCode::OK,
        cookies.add(Cookie::new(
            state.config.auth.session_cookie_name.clone(),
            session_token.clone(),
        )),
        Json(NewSessionResponse {
            user_id: user_id.to_string(),
            session_token,
        }),
    ))
}

pub async fn logout(State(_): State<SharedState>, Session(session): Session) -> impl IntoResponse {
    Json(format!("{:?}", session))
}

pub async fn logout_everywhere(State(_): State<SharedState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
