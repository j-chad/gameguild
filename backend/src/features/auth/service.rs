use super::queries::{find_user_id_by_email, find_user_id_by_username, new_user};
use super::schemas::RegisterRequest;
use super::utils::password::hash_password;
use crate::error::AppError;
use crate::features::auth::errors::AuthError;
use crate::features::auth::{queries, utils};
use chrono::Duration;
use std::net::IpAddr;

const SESSION_TOKEN_SIZE: usize = 64;
const SESSION_EXPIRATION: Duration = Duration::weeks(2);

pub async fn register_user(
    pool: &sqlx::PgPool,
    data: &RegisterRequest,
) -> Result<uuid::Uuid, AppError> {
    if find_user_id_by_email(pool, &data.email).await?.is_some() {
        return Err(AuthError::UserAlreadyExists(data.email.clone()).into());
    }

    if find_user_id_by_username(pool, &data.username)
        .await?
        .is_some()
    {
        return Err(AuthError::UserAlreadyExists(data.username.clone()).into());
    }

    let user_id = uuid::Uuid::now_v7();
    let password_hash = hash_password(&data.password).map_err(|e| {
        tracing::error!(err=?e, "failed to hash password");
        AuthError::PasswordHashingFailed
    })?;

    new_user(pool, user_id, &data.email, &data.username, &password_hash)
        .await
        .inspect_err(|err| {
            tracing::error!(err=?err, "failed to insert user into database");
        })?;

    tracing::debug!(id = user_id.to_string(), "user registered successfully");
    Ok(user_id)
}

pub async fn new_session(
    pool: &sqlx::PgPool,
    user_id: uuid::Uuid,
    ip_addr: Option<IpAddr>,
    user_agent: Option<String>,
) -> Result<String, AppError> {
    let id = uuid::Uuid::new_v4();
    let token = utils::session_token::new(SESSION_TOKEN_SIZE)?;
    let expires_at = chrono::Utc::now() + SESSION_EXPIRATION;

    queries::new_session(pool, id, user_id, &token, expires_at, user_agent, ip_addr)
        .await
        .inspect_err(|err| {
            tracing::error!(err = ?err, "failed to create new session");
        })?;

    Ok(token)
}
