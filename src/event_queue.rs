use crate::action::Action;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct EventQueue {
    pub actions: VecDeque<Action>,
}

impl EventQueue {
    pub fn schedule_action(&mut self, action: Action) {
        self.actions.push_back(action);
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn get_next_action(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }
}
