use sqlx::PgPool;

pub async fn find_user_id_by_email_or_username(
    pool: &PgPool,
    email: &str,
    username: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT id, email, username FROM users WHERE email = $1 OR USERNAME = $2 LIMIT 1",
        email,
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| r.id))
}
