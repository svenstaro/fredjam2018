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
            Action::Command(command) => {
                // TODO Maybe there's a better approach to finding the current room...
                match state.current_room {
                    RoomType::Cryobay => {
                        // TODO replace command by proper enum.
                        if command == &"use lever" {
                            if !self.lever {
                                event_queue.schedule_action(Action::Message(
                                        String::from(
                                            "You pull the lever, it flips down with a screeching noize.",
                                            ),
                                            GameEventType::Success,
                                            ));
                                self.lever = true;
                            } else {
                                event_queue.schedule_action(Action::Message(
                                    String::from("There is no point in pulling this back up."),
                                    GameEventType::Failure,
                                ));
                            }
                            ActionHandled::Handled
                        } else if command == &"look around" {
                            event_queue.schedule_action(Action::Message(
                                    String::from("The room is empty, there is just some lever and a solid door with no handle."),
                                    GameEventType::Normal,
                                    ));
                            ActionHandled::Handled
                        } else if command == &"use door" {
                            if self.lever {
                                event_queue.schedule_action(Action::Message(
                                    String::from("You open the door and pass through."),
                                    GameEventType::Success,
                                ));
                                event_queue.schedule_action(Action::Leave(RoomType::Cryobay));
                                event_queue.schedule_action(Action::Enter(RoomType::SlushLobby));
                                ActionHandled::Handled
                            } else {
                                event_queue.schedule_action(Action::Message(
                                        String::from("No matter how hard you push the door, it does not even move an inch."),
                                        GameEventType::Failure,
                                        ));
                                ActionHandled::Handled
                            }
                        } else {
                            ActionHandled::NotHandled
                        }
                    }
                    _ => ActionHandled::NotHandled,
                }
            }
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
