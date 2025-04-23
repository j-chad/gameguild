// src/main.rs
mod app;
mod config;
mod db;
mod error;
mod state;

use app::create_app;
use config::load_config;
use db::connect_db;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = load_config()?;
    let db = connect_db(&config.database_url).await?;
    let app_state = AppState::new(db.clone(), config);

    let app = create_app(app_state);

    let addr = "127.0.0.1:3000".parse()?;
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
