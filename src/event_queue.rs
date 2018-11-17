use crate::action::Action;
use crate::timer::{Timer, TimerType};
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

    pub fn schedule_actions(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.actions.push_back(action);
        }
    }

    pub fn schedule_timer(&mut self, timer: Timer) {
        self.timers.push(timer);
    }

    pub fn schedule_timers(&mut self, timers: Vec<Timer>) {
        for timer in timers {
            self.timers.push(timer);
        }
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

    pub fn get_timers(&self, timer_type: TimerType) -> Vec<Timer> {
        self.timers
            .clone()
            .into_iter()
            .filter(|timer| timer.timer_type == timer_type)
            .collect::<Vec<Timer>>()
    }

    pub fn emplace_timers(&mut self, timer_type: TimerType, emplacement: Vec<Timer>) {
        self.timers.retain(|timer| timer.timer_type != timer_type);
        self.timers.extend(emplacement);
    }
}
