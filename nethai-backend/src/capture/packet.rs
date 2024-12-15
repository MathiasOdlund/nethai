use pcap::{Active, Capture, Device};
use std::error::Error;
use tokio::sync::mpsc;

pub struct PacketCapture {
    capture: Option<Capture<Active>>,
    running: bool,
}

impl PacketCapture {
    pub fn new() -> Self {
        Self {
            capture: None,
            running: false,
        }
    }

    pub fn start_capture(&mut self, device: &Device) -> Result<(), Box<dyn Error>> {
        let cap = Capture::from_device(device.clone())?
            .promisc(true)
            .snaplen(65535)
            .timeout(1000)
            .open()?;

        self.capture = Some(cap);
        self.running = true;
        Ok(())
    }

    pub fn stop_capture(&mut self) {
        self.running = false;
        self.capture = None;
    }

    pub async fn capture_loop(
        &mut self,
        packet_sender: mpsc::Sender<Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        let cap = self.capture.as_mut().ok_or("Capture not initialized")?;

        while self.running {
            if let Ok(packet) = cap.next_packet() {
                let packet_data = packet.data.to_vec();
                if let Err(_) = packet_sender.send(packet_data).await {
                    break;
                }
            }
        }

        Ok(())
    }
}
