use crate::services;
use crate::services::health::HealthReport;
use crate::state::SharedState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{response::Json, routing::get};

pub fn build_router() -> axum::Router<SharedState> {
    axum::Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/check", post(health_check))
}

async fn health_check(State(state): State<SharedState>) -> (StatusCode, Json<HealthReport>) {
    let report = services::health::run_health_checks(&state).await;

    let status = match report.result {
        services::health::HealthCheckResult::Ok => StatusCode::OK,
        services::health::HealthCheckResult::Failed(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (status, Json(report))
}
