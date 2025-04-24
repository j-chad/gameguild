// src/main.rs
mod app;
mod config;
mod db;
mod error;
mod logging;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Settings::new()?;
    logging::init_tracing(&config.logging)?;

    tracing::info!("Starting application in {} mode", config.env);
    // let db = connect_db(&config.database_url).await?;
    // let app_state = AppState::new(db.clone(), config);
    //
    // let app = create_app(app_state);
    //
    // let addr = "127.0.0.1:3000".parse()?;
    // tracing::info!("Listening on {}", addr);
    // axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
