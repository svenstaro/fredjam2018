use crate::entities::Item;
use crate::game_event::GameEventType;
use crate::room::{Room, RoomType};
use crate::EventQueue;
use crate::{Action, ActionHandled, State};

#[derive(Debug)]
pub struct CryobayRoom {
    pub visited: bool,
    pub lever: bool,
    pub crowbar: bool,
    pub casket_locked: bool,
}

impl CryobayRoom {
    pub fn new() -> CryobayRoom {
        CryobayRoom {
            visited: false,
            lever: false,
            crowbar: true,
            casket_locked: true,
        }
    }
}

impl Room for CryobayRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled {
        match action {
            Action::PickUpCrowbar => {
                if self.crowbar {
                    state.player.items.push(Item::Crowbar);
                    self.crowbar = false;
                    event_queue.schedule_action(Action::Message(
                        String::from("You pick up the crowbar."),
                        GameEventType::Success,
                    ));

                    ActionHandled::Handled
                } else {
                    event_queue.schedule_action(Action::Message(
                        String::from("You already have the crowbar."),
                        GameEventType::Failure,
                    ));

                    ActionHandled::Handled
                }
            }
            Action::UseCasket => {
                event_queue.schedule_action(Action::Message(
                    "\"Reboot initiated.\" Those are the last words you hear as you slip back into cryosleep once again.".into(),
                    GameEventType::Success
                ));

                event_queue.schedule_action(Action::GameOver);

                ActionHandled::Handled
            }
            _ => ActionHandled::NotHandled,
        }
    }

    fn is_opened(&self) -> bool {
        true
    }

    fn open(&mut self) {}

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }
}
