#![allow(dead_code)]

use std::time::{Duration, Instant};

pub struct FrameTimer {
    last: Instant,
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
        }
    }

    pub fn delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let dt = now - self.last;
        self.last = now;
        dt.as_secs_f32()
    }
}
