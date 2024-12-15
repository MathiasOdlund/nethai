mod api;
mod capture;
mod models;
mod storage;

use crate::capture::stats::NetworkStats;
use crate::capture::PacketCapture;
use axum::http::{HeaderValue, Method};
use axum::{Extension, Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

// AppState will hold our shared state
pub struct AppState {
    capture_active: bool,
    capture: Arc<Mutex<Option<PacketCapture>>>,
    stats: Arc<RwLock<NetworkStats>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            capture_active: false,
            capture: Arc::new(Mutex::new(None)),
            stats: Arc::new(RwLock::new(NetworkStats::new())),
        }
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // Initialize shared state
    let state = Arc::new(RwLock::new(AppState::default()));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    // Create router with routes
    let router = Router::new()
        .merge(api::routes::create_routes())
        .layer(Extension(state))
        .layer(cors);

    Ok(router.into())
}
