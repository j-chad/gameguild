use crate::state::SharedState;
use axum::{Extension, Router};
use tower_http::trace::TraceLayer;

pub fn create_app(state: SharedState) -> Router {
    Router::new()
        .nest("/api", Router::new())
        .layer(Extension(state))
        .layer(TraceLayer::new_for_http())
}
