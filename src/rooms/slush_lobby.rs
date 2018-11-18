use crate::room::Room;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};
use crate::game_event::GameEventType;
use crate::entities::Item;

// Second room room, locked per default, lever needs to be pulled.
#[derive(Debug)]
pub struct SlushLobbyRoom {
    pub visited: bool,
}

impl SlushLobbyRoom {
    pub fn new() -> SlushLobbyRoom {
        SlushLobbyRoom {
            visited: false,
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
            Action::UseCrowbar => {
                if state.player.has_item(Item::Crowbar) {
                    event_queue.schedule_action(Action::OpenCorridor);
                    ActionHandled::Handled
                } else {
                    event_queue.schedule_action(Action::Message(
                            String::from("You don't have a crowbar."),
                            GameEventType::Failure,
                            ));

                    ActionHandled::Handled
                }
            }
            Action::UseKeycard => {
                if state.player.has_item(Item::Crowbar) {
                    event_queue.schedule_action(Action::OpenCorridor);
                    ActionHandled::Handled
                } else {
                    event_queue.schedule_action(Action::Message(
                            String::from("You don't have a keycard."),
                            GameEventType::Failure,
                            ));

                    ActionHandled::Handled
                }
            }
            _ => ActionHandled::NotHandled
        }
    }

    fn is_opened(&self) -> bool {
        true
    }

    fn open(&mut self) {
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }
}
