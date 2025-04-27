use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        AppState { db }
    }
}

pub type SharedState = Arc<AppState>;
