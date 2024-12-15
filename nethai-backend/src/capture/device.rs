use pcap::{Capture, Device};
use std::error::Error;

pub struct DeviceManager {
    current_device: Option<Device>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            current_device: None,
        }
    }

    pub fn get_available_devices() -> Result<Vec<Device>, pcap::Error> {
        Device::list()
    }

    pub fn get_default_device() -> Result<Device, pcap::Error> {
        Device::lookup()?.ok_or(pcap::Error::PcapError(
            "No default device found".to_string(),
        ))
    }

    pub fn select_device(&mut self, device: Device) {
        self.current_device = Some(device);
    }

    pub fn get_current_device(&self) -> Option<&Device> {
        self.current_device.as_ref()
    }
}
