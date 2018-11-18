use crate::game_event::GameEventType;
use crate::room::Room;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};
use crate::entities::Item;

#[derive(Debug)]
pub struct CorridorRoom {
    pub visited: bool,
    pub keycard: bool,
    pub opened: bool,
}

impl CorridorRoom {
    pub fn new() -> CorridorRoom {
        CorridorRoom {
            visited: false,
            keycard: true,
            opened: false,
        }
    }
}

impl Room for CorridorRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled {
        match action {
            Action::PickUpKeycard => {
                if self.keycard {
                    state.player.items.push(Item::KeyCard);
                    event_queue.schedule_action(Action::Message(
                        String::from("You pick up the key card."),
                        GameEventType::Failure,
                    ));
                } else {
                    event_queue.schedule_action(Action::Message(
                        String::from("You already have the key card."),
                        GameEventType::Failure,
                    ));
                }
                ActionHandled::Handled
            }
            _ => ActionHandled::NotHandled
        }
    }

    fn is_opened(&self) -> bool {
        self.opened
    }

    fn open(&mut self) {
        self.opened = true
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }
}
