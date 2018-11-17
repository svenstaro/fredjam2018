use std::collections::HashMap;
use crate::action::Action;

#[derive(Debug)]
pub struct Timer {
    label: String,
    elapsed: u64,
    duration: u64,
    timings: HashMap<u8, Action>,
}

impl Timer {
    pub fn new(label: &str, elapsed: u64, duration: u64, timings: HashMap<u8, Action>) -> Self {
        Timer {
            label: label.to_string(),
            elapsed,
            duration,
            timings,
        }
    }

    pub fn tick(&mut self, dt: u64) {
        self.elapsed += dt;
    }
}
