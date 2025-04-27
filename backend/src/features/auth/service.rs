use super::queries::{find_user_id_by_email, find_user_id_by_username, insert_user};
use super::schemas::RegisterRequest;
use super::utils::password::hash_password;
use crate::error::AppError;
use crate::features::auth::errors::AuthError;

pub async fn register_user(
    pool: &sqlx::PgPool,
    data: &RegisterRequest,
) -> Result<uuid::Uuid, AppError> {
    if let Some(_) = find_user_id_by_email(pool, &data.email).await? {
        return Err(AuthError::UserAlreadyExists(data.email.clone()).into());
    }

    if let Some(_) = find_user_id_by_username(pool, &data.username).await? {
        return Err(AuthError::UserAlreadyExists(data.username.clone()).into());
    }

    let user_id = uuid::Uuid::now_v7();
    let password_hash = hash_password(&data.password).map_err(|e| {
        tracing::error!(err=?e, "failed to hash password");
        AuthError::PasswordHashingFailed
    })?;

    insert_user(pool, user_id, &data.email, &data.username, &password_hash)
        .await
        .map_err(|_| anyhow::anyhow!("Failed to insert user"))?;

    Ok(user_id)
}
