use crate::action::Action;
use crate::event_queue::EventQueue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Timer {
    pub label: String,
    pub elapsed: u64,
    pub duration: u64,
    pub action: Action,
    pub is_visual: bool,
}

impl Timer {
    pub fn new(label: &str, elapsed: u64, duration: u64, action: Action, is_visual: bool) -> Self {
        Timer {
            label: label.to_string(),
            elapsed,
            duration,
            action,
            is_visual,
        }
    }

    pub fn tick(&mut self, dt: u64) {
        self.elapsed += dt;
    }

    pub fn current_percent(&self) -> u8 {
        (self.duration / self.elapsed) as u8
    }

    pub fn is_done(&self) -> bool {
        self.current_percent() > 100
    }
}
