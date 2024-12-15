use crate::capture::stats::NetworkStats;
use crate::capture::{self, DeviceManager, PacketCapture};
use crate::AppState;
use axum::{debug_handler, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};

#[derive(Serialize)]
pub struct StatsResponse {
    packets_received: u64,
    bytes_received: u64,
    duration_secs: f64,
}

#[derive(Serialize)]
pub struct CaptureStatus {
    is_active: bool,
    current_device: Option<String>,
}

#[derive(Deserialize)]
pub struct SelectDeviceRequest {
    name: String,
}

#[derive(Serialize)]
pub struct NetworkInterface {
    name: String,
    description: Option<String>,
    addresses: Vec<InterfaceAddress>,
    is_up: bool,
    is_running: bool,
    is_wireless: bool,
}

#[derive(Serialize)]
pub struct InterfaceAddress {
    address: String,
    netmask: Option<String>,
    broadcast: Option<String>,
}

#[debug_handler]
pub async fn get_status(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let state = state.read().await;
    Json(format!("Capture active: {}", state.capture_active))
}

pub async fn get_interfaces() -> impl IntoResponse {
    match capture::device::DeviceManager::get_available_devices() {
        Ok(devices) => {
            let interfaces: Vec<NetworkInterface> = devices
                .into_iter()
                .map(|d| {
                    let flags = d.flags;
                    NetworkInterface {
                        name: d.name,
                        description: d.desc,
                        addresses: d
                            .addresses
                            .into_iter()
                            .map(|addr| InterfaceAddress {
                                address: addr.addr.to_string(),
                                netmask: addr.netmask.map(|n| n.to_string()),
                                broadcast: addr.broadcast_addr.map(|b| b.to_string()),
                            })
                            .collect(),
                        is_up: flags.is_up(),
                        is_running: flags.is_running(),
                        is_wireless: flags.is_wireless(),
                    }
                })
                .collect();
            Json(interfaces)
        }
        Err(e) => Json(vec![]),
    }
}

#[debug_handler]
pub async fn start_capture(
    Extension(state): Extension<Arc<RwLock<AppState>>>,
) -> impl IntoResponse {
    let mut state = state.write().await;

    // Get default device
    let device = match DeviceManager::get_default_device() {
        Ok(dev) => dev,
        Err(e) => return Json(format!("Error getting default device: {}", e)),
    };

    // Initialize capture
    let mut capture = PacketCapture::new();
    if let Err(e) = capture.start_capture(&device) {
        return Json(format!("Error starting capture: {}", e));
    }

    // Setup channel for packet processing
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(100);
    let stats = state.stats.clone();

    // Spawn packet processing task
    tokio::spawn(async move {
        while let Some(packet_data) = rx.recv().await {
            let mut stats = stats.write().await;
            stats.update(packet_data.len());
        }
    });

    // Store capture and start capture loop
    let capture_arc = Arc::new(Mutex::new(Some(capture)));
    state.capture = capture_arc.clone();
    state.capture_active = true;

    Json(format!("Capture started successfully"))
}

#[debug_handler]
pub async fn stop_capture(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let mut state = state.write().await;
    state.capture_active = false;
    Json(format!("Capture stopped"))
}

pub async fn get_stats(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let state = state.read().await;
    let stats = state.stats.read().await;
    let (packets, bytes, duration) = stats.get_stats();

    Json(StatsResponse {
        packets_received: packets,
        bytes_received: bytes,
        duration_secs: duration.as_secs_f64(),
    })
}

pub async fn reset_stats(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let state = state.read().await;
    let mut stats = state.stats.write().await;
    stats.reset();
    Json(format!("Statistics reset successfully"))
}

pub async fn get_capture_status(
    Extension(state): Extension<Arc<RwLock<AppState>>>,
) -> impl IntoResponse {
    let state = state.read().await;
    let capture = state.capture.lock().await;

    Json(CaptureStatus {
        is_active: state.capture_active,
        current_device: None, // You could enhance this to store the current device name
    })
}

pub async fn select_device(
    Extension(state): Extension<Arc<RwLock<AppState>>>,
    Json(request): Json<SelectDeviceRequest>,
) -> impl IntoResponse {
    // Get available devices
    let devices = match DeviceManager::get_available_devices() {
        Ok(devices) => devices,
        Err(e) => return Json(format!("Error getting devices: {}", e)),
    };

    // Find the device with matching name
    let selected_device = devices.into_iter().find(|d| d.name == request.name);

    match selected_device {
        Some(device) => {
            let mut state = state.write().await;
            if let Some(capture) = state.capture.lock().await.as_mut() {
                capture.stop_capture();
            }

            // Initialize new capture with selected device
            let mut capture = PacketCapture::new();
            if let Err(e) = capture.start_capture(&device) {
                return Json(format!(
                    "Error starting capture with selected device: {}",
                    e
                ));
            }

            state.capture = Arc::new(Mutex::new(Some(capture)));
            state.capture_active = true;

            Json(format!(
                "Successfully selected and started capture on device: {}",
                request.name
            ))
        }
        None => Json(format!("Device with name {} not found", request.name)),
    }
}
