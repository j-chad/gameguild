use super::queries::{find_user_id_by_email, find_user_id_by_username, new_user};
use super::schemas::RegisterRequest;
use super::utils::password::hash_password;
use crate::error::AppError;
use crate::features::auth::errors::AuthError;
use crate::features::auth::{queries, utils};
use anyhow::anyhow;
use chrono::Duration;
use sqlx::PgPool;
use std::net::IpAddr;
use std::sync::LazyLock;

const SESSION_TOKEN_SIZE: usize = 64;
const SESSION_EXPIRATION: Duration = Duration::weeks(2);

pub async fn register_user(pool: &PgPool, data: &RegisterRequest) -> Result<uuid::Uuid, AppError> {
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
        anyhow!("failed to hash password")
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

// Used to prevent time-based attacks by always returning a valid hash
static FAKE_PASSWORD_HASH: LazyLock<String> =
    LazyLock::new(|| hash_password("fake_password_123").expect("Failed to create fake hash"));

pub(crate) async fn login_user(
    pool: &PgPool,
    username: Option<&str>,
    email: Option<&str>,
    password: &String,
) -> Result<uuid::Uuid, AppError> {
    let (user_id, password_hash) =
        queries::find_user_id_and_pw_by_username_or_email(pool, username, email)
            .await?
            .unwrap_or((uuid::Uuid::nil(), FAKE_PASSWORD_HASH.to_string()));

    utils::password::validate_password(password, &password_hash).map_err(|err| -> AppError {
        if err == argon2::password_hash::Error::Password {
            AuthError::InvalidCredentials.into()
        } else {
            tracing::error!(err = ?err, "failed to validate password");
            anyhow::anyhow!("failed to validate password").into()
        }
    })?;

    // If the user_id is nil, it means the user was not found
    if user_id == uuid::Uuid::nil() {
        return Err(AuthError::InvalidCredentials.into());
    }

    Ok(user_id)
}
