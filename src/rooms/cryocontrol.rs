use crate::room::Room;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};

#[derive(Debug)]
pub struct Cryocontrol {
    pub visited: bool,
    pub lever: bool,
}

impl Cryocontrol {
    pub fn new() -> Cryocontrol {
        Cryocontrol {
            visited: false,
            lever: false,
        }
    }
}

impl Room for Cryocontrol {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled {
        ActionHandled::NotHandled
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }
}
