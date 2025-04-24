use crate::config;
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{debug, info};

pub async fn connect_db(config: &config::PostgreSQLSettings) -> Result<PgPool> {
    debug!("Connecting to postgresql");
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections) // Set the max pool size to 10
        .connect(&config.url)
        .await?;
    info!("Successfully connected to postgresql");

    Ok(pool)
}
