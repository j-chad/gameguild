use crate::queries::auth::{find_user_id_by_email, find_user_id_by_username, insert_user};
use crate::schemas::auth::RegisterRequest;
use crate::utils::password::hash_password;

pub async fn register_user(
    pool: &sqlx::PgPool,
    data: &RegisterRequest,
) -> Result<uuid::Uuid, anyhow::Error> {
    // check if email already exists
    if let Some(_) = find_user_id_by_email(pool, &data.email).await? {
        return Err(anyhow::anyhow!("Email already exists"));
    }

    // check if username already exists
    if let Some(_) = find_user_id_by_username(pool, &data.username).await? {
        return Err(anyhow::anyhow!("Username already exists"));
    }

    // create user
    let user_id = uuid::Uuid::now_v7();
    let password_hash = hash_password(&data.password)?;

    insert_user(pool, user_id, &data.email, &data.username, &password_hash)
        .await
        .map_err(|_| anyhow::anyhow!("Failed to insert user"))?;

    Ok(user_id)
}
