use crate::schemas::auth::RegisterRequest;

pub async fn register_user(
    pool: &sqlx::PgPool,
    data: RegisterRequest,
) -> Result<uuid::Uuid, sqlx::Error> {
    Err(sqlx::Error::RowNotFound)
}
