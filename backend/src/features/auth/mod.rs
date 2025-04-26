use crate::state::SharedState;
use axum::routing::post;

mod handlers;
mod queries;
mod schemas;
mod service;
mod utils;

pub fn build_router() -> axum::Router<SharedState> {
    axum::Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register))
        .route("/logout", post(handlers::logout))
        .route("/logout-everywhere", post(handlers::logout_everywhere))
}
