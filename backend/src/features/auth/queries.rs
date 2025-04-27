use sqlx::types::ipnet::IpNet;
use sqlx::PgPool;

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
    ip_address: Option<IpNet>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO sessions (id, user_id, token, expires_at, user_agent, ip_address) VALUES ($1, $2, $3, $4, $5, $6)",
        id,
        user_id,
        token,
        expires_at,
        user_agent,
        ip_address
    )
    .execute(pool)
    .await?;

    Ok(())
}
