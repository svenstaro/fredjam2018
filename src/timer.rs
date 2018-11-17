use crate::action::Action;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Timer {
    pub label: String,
    pub elapsed: u64,
    pub duration: u64,
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
