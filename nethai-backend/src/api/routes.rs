use super::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/status", get(handlers::get_capture_status))
        .route("/interfaces", get(handlers::get_interfaces))
        .route("/capture/start", post(handlers::start_capture))
        .route("/capture/stop", post(handlers::stop_capture))
        .route("/stats", get(handlers::get_stats))
        .route("/stats/reset", post(handlers::reset_stats))
        .route("/device/select", post(handlers::select_device))
}
