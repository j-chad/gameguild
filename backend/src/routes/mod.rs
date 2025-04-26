use crate::state::SharedState;
use axum::Router;

mod health;

pub fn build_router() -> Router<SharedState> {
    Router::new().nest("/v1", build_v1_router())
}

pub fn build_v1_router() -> Router<SharedState> {
    Router::new().nest("/health", health::build_router())
}
