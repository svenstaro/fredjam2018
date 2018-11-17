use std::collections::HashMap;
use crate::action::Action;

#[derive(Debug)]
pub struct Timer {
    timings: HashMap<u8, Action>,
}

impl Timer {
    pub fn new(timings: HashMap<u8, Action>) -> Self {
        Timer {
            timings
        }
    }

    pub fn tick(&mut self, dt: u64) {
    }
}
