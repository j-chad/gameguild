use crate::handlers::auth::{login, logout, logout_everywhere, register};
use crate::state::SharedState;
use axum::routing::post;

pub fn build_router() -> axum::Router<SharedState> {
    axum::Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
        .route("/logout-everywhere", post(logout_everywhere))
}
