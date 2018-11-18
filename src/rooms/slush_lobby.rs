use crate::room::Room;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};

// Second room room, locked per default, lever needs to be pulled.
#[derive(Debug)]
pub struct SlushLobbyRoom {
    pub visited: bool,
    pub door_opened: bool,
    pub shaft_opened: bool,
}

impl SlushLobbyRoom {
    pub fn new() -> SlushLobbyRoom {
        SlushLobbyRoom {
            visited: false,
            door_opened: false,
            shaft_opened: false
        }
    }
}

impl Room for SlushLobbyRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled {
        match action {
            _ => return ActionHandled::NotHandled,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }
}
