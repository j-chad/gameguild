use super::schemas::{LoginRequest, NewSessionResponse, RegisterRequest};
use super::{queries, service};
use crate::error::AppError;
use crate::features::auth::middleware::Session;
use crate::features::auth::utils::cookie::new_session_cookie;
use crate::state::SharedState;
use axum::extract::{ConnectInfo, State};
use axum::http::header::USER_AGENT;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
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
    let session_cookie = new_session_cookie(&state.config.auth, &session_token);

    Ok((
        StatusCode::CREATED,
        cookies.add(session_cookie),
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
    let session_cookie = new_session_cookie(&state.config.auth, &session_token);

    Ok((
        StatusCode::OK,
        cookies.add(session_cookie),
        Json(NewSessionResponse {
            user_id: user_id.to_string(),
            session_token,
        }),
    ))
}

pub async fn logout(
    State(state): State<SharedState>,
    Session(session): Session,
    cookies: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    queries::delete_session(&state.db, &session.id).await?;

    let removal_cookie = new_session_cookie(&state.config.auth, "");
    Ok((StatusCode::NO_CONTENT, cookies.remove(removal_cookie)))
}

pub async fn logout_everywhere(
    State(state): State<SharedState>,
    Session(session): Session,
    cookies: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    queries::delete_all_sessions(&state.db, &session.user_id).await?;

    let removal_cookie = new_session_cookie(&state.config.auth, "");
    Ok((StatusCode::NO_CONTENT, cookies.remove(removal_cookie)))
}
