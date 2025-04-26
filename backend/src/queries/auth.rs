// pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<Uuid>, sqlx::Error> {
//     let record = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
//         .fetch_optional(pool)
//         .await?;
//
//     Ok(record.map(|r| r.id))
// }
