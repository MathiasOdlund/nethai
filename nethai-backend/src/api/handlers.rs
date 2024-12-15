use crate::capture;
use crate::AppState;
use axum::{debug_handler, response::IntoResponse, Extension, Json};
use std::sync::Arc;
use tokio::sync::RwLock;

#[debug_handler]
pub async fn get_status(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let state = state.read().await;
    Json(format!("Capture active: {}", state.capture_active))
}

pub async fn get_interfaces() -> impl IntoResponse {
    match capture::device::DeviceManager::get_available_devices() {
        Ok(devices) => Json(format!("Available devices: {:?}", devices)),
        Err(e) => Json(format!("Error: {}", e)),
    }
}

#[debug_handler]
pub async fn start_capture(
    Extension(state): Extension<Arc<RwLock<AppState>>>,
) -> impl IntoResponse {
    let mut state = state.write().await;
    state.capture_active = true;
    Json("Capture started")
}

#[debug_handler]
pub async fn stop_capture(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let mut state = state.write().await;
    state.capture_active = false;
    Json("Capture stopped")
}

pub async fn get_stats() -> impl IntoResponse {
    Json("Stats will be implemented")
}
