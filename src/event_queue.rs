use crate::action::Action;
use crate::timer::Timer;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct EventQueue {
    pub actions: VecDeque<Action>,
    pub timers: Vec<Timer>,
}

impl EventQueue {
    pub fn schedule_action(&mut self, action: Action) {
        self.actions.push_back(action);
    }

    pub fn schedule_timer(&mut self, timer: Timer) {
        self.timers.push(timer);
    }

    pub fn tick(&mut self, dt: u64) {
        for timer in &mut self.timers {
            timer.tick(dt);
        }

        let cloned_timers = self.timers.clone();
        for timer in cloned_timers {
            if timer.is_done() {
                self.schedule_action(timer.action);
            }
        }

        self.timers.retain(|x| !x.is_done());
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn get_next_action(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }
}
