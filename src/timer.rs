use crate::action::Action;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerType {
    EnemyAttack,
    Oxygen,
    Storytime,
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub timer_type: TimerType,
    pub label: String,
    pub elapsed: u64,
    pub duration: u64,
    pub action: Action,
    pub is_visual: bool,
}

impl Timer {
    pub fn new(timer_type: TimerType, label: &str, elapsed: u64, duration: u64, action: Action, is_visual: bool) -> Self {
        Timer {
            timer_type,
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
        (self.elapsed * 100 / self.duration) as u8
    }

    pub fn is_done(&self) -> bool {
        self.current_percent() > 100
    }
}
