use std::time::{Duration, Instant};

pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Timer { start: Instant::now() }
    }
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}
