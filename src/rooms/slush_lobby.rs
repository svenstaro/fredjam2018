use crate::room::Room;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};
use crate::game_event::GameEventType;
use crate::entities::Item;

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
            Action::UseCrowbar => {
                if !self.shaft_opened && state.player.has_item(Item::Crowbar) {
                    self.shaft_opened = true;
                    event_queue.schedule_action(Action::Message(
                            String::from("You smash open the ventilation shaft cover with your crowbar."),
                            GameEventType::Success,
                            ));

                    ActionHandled::Handled
                } else {
                    event_queue.schedule_action(Action::Message(
                            String::from("The ventilation shaft is already open."),
                            GameEventType::Failure,
                            ));

                    ActionHandled::Handled
                }
            }
            Action::UseKeycard => {
                if !self.shaft_opened && state.player.has_item(Item::Crowbar) {
                    self.door_opened = true;
                    event_queue.schedule_action(Action::Message(
                            String::from("You open the cryo control door. SSswsschh"),
                            GameEventType::Success,
                            ));

                    ActionHandled::Handled
                } else {
                    event_queue.schedule_action(Action::Message(
                            String::from("The cryo control door is already open."),
                            GameEventType::Failure,
                            ));

                    ActionHandled::Handled
                }
            }
            _ => ActionHandled::NotHandled
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }
}
