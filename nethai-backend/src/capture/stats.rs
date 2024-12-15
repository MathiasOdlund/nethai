use std::time::{Duration, Instant};

pub struct NetworkStats {
    start_time: Instant,
    packets_received: u64,
    bytes_received: u64,
    last_update: Instant,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            packets_received: 0,
            bytes_received: 0,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self, packet_size: usize) {
        self.packets_received += 1;
        self.bytes_received += packet_size as u64;
        self.last_update = Instant::now();
    }

    pub fn get_stats(&self) -> (u64, u64, Duration) {
        let elapsed = self.last_update.duration_since(self.start_time);
        (self.packets_received, self.bytes_received, elapsed)
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.packets_received = 0;
        self.bytes_received = 0;
        self.last_update = Instant::now();
    }
}
