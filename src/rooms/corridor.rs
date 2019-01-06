use crate::entities::Item;
use crate::game_event::GameEventType;
use crate::room::Room;
use crate::EventQueue;
use crate::{Action, ActionHandled, State, RoomType};
use strum::EnumProperty;

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
            Action::OpenCorridor => {
                if !self.opened {
                    self.opened = true;
                    event_queue.schedule_action(Action::Message(
                        String::from(
                            "You smash open the ventilation shaft cover with your crowbar.",
                        ),
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
            Action::PickUpKeycard => {
                if self.keycard {
                    state.player.items.push(Item::KeyCard);
                    event_queue.schedule_action(Action::Message(
                        String::from("You pick up the keycard."),
                        GameEventType::Success,
                    ));
                } else {
                    event_queue.schedule_action(Action::Message(
                        String::from("You already have the keycard."),
                        GameEventType::Failure,
                    ));
                }
                ActionHandled::Handled
            }
            Action::ShowEnterText => {
                if state.get_current_enemy(RoomType::Corridor).is_none() {
                    event_queue.schedule_action(Action::Message(
                        "You see the rat's keycard lying around in the corner.".into(),
                        GameEventType::Normal
                    ))
                }
                ActionHandled::NotHandled
            }
            _ => ActionHandled::NotHandled,
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
