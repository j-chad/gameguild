use super::models;
use sqlx::types::ipnet::IpNet;
use sqlx::PgPool;
use std::net::IpAddr;
use uuid::Uuid;

pub async fn find_user_id_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let record = sqlx::query!("SELECT id FROM users WHERE email = $1 LIMIT 1", email,)
        .fetch_optional(pool)
        .await?;

    Ok(record.map(|r| r.id))
}

pub async fn find_user_id_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let record = sqlx::query!("SELECT id FROM users WHERE username = $1 LIMIT 1", username,)
        .fetch_optional(pool)
        .await?;

    Ok(record.map(|r| r.id))
}

pub async fn new_user(
    pool: &PgPool,
    id: uuid::Uuid,
    email: &str,
    username: &str,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (id, email, username, password_hash) VALUES ($1, $2, $3, $4)",
        id,
        email,
        username,
        password_hash
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn new_session(
    pool: &PgPool,
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    token: &str,
    expires_at: chrono::DateTime<chrono::Utc>,
    user_agent: Option<String>,
    ip_address: Option<IpAddr>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO sessions (id, user_id, token, expires_at, user_agent, ip_address) VALUES ($1, $2, $3, $4, $5, $6)",
        id,
        user_id,
        token,
        expires_at,
        user_agent,
        ip_address.map(IpNet::from)
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_user_id_and_pw_by_username_or_email(
    pool: &PgPool,
    username: Option<&str>,
    email: Option<&str>,
) -> Result<Option<(uuid::Uuid, String)>, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE (username = $1 OR email = $2) LIMIT 1",
        username,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| (r.id, r.password_hash)))
}

pub async fn get_session_by_token_and_update_access_time(
    pool: &PgPool,
    token: &str,
) -> Result<Option<models::Session>, sqlx::Error> {
    let record = sqlx::query!(
        r#"
    UPDATE sessions
    SET last_seen_at = CURRENT_TIMESTAMP
    WHERE token = $1
    RETURNING id, user_id, created_at, last_seen_at, expires_at, user_agent, ip_address
    "#,
        token
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| models::Session {
        id: r.id,
        user_id: r.user_id,
        created_at: r.created_at,
        expires_at: r.expires_at,
        last_seen_at: r.last_seen_at,
        user_agent: r.user_agent,
        ip_address: r.ip_address.map(|net| net.addr()),
    }))
}

pub async fn delete_session(pool: &PgPool, session_id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE id = $1", session_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub(crate) async fn delete_all_sessions(pool: &PgPool, user_id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE user_id = $1", user_id)
        .execute(pool)
        .await?;

    Ok(())
}
