use crate::config;
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect_db(config: &config::PostgreSQLSettings) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections) // Set the max pool size to 10
        .connect(&config.url)
        .await?;

    Ok(pool)
}
