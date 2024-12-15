mod api;
mod capture;
mod models;
mod storage;

use crate::capture::PacketCapture;
use axum::{Extension, Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

// AppState will hold our shared state
pub struct AppState {
    capture_active: bool,
    capture: Arc<Mutex<Option<PacketCapture>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            capture_active: false,
            capture: Arc::new(Mutex::new(None)),
        }
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // Initialize shared state
    let state = Arc::new(RwLock::new(AppState::default()));

    // Create router with routes
    let router = Router::new()
        .merge(api::routes::create_routes())
        .layer(Extension(state));

    Ok(router.into())
}
