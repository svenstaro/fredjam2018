use std::fmt::Debug;
use crate::EventQueue;
use crate::{Action, GameEventType, State};

pub trait Room: Debug {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> bool;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RoomType {
    WakeUp,
    Locked,
}

// Initial room.
#[derive(Debug)]
pub struct WakeUpRoom {
    pub lever: bool,
}

impl Room for WakeUpRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> bool {
        match action {
            Action::Enter(room) => match room {
                RoomType::WakeUp => {
                    event_queue.schedule_action(Action::Message(
                        String::from("Welcome to W.O.R.L.D."),
                        GameEventType::Success,
                    ));
                    true
                }
                _ => false,
            },
            Action::Command(command) => {
                // TODO Maybe there's a better approach to finding the current room...
                match state.current_room {
                    RoomType::WakeUp => {
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
                            true
                        } else if command == &"look around" {
                            event_queue.schedule_action(Action::Message(
                                    String::from("The room is empty, there is just some lever and a solid door with no handle."),
                                    GameEventType::Normal,
                                    ));
                            true
                        } else if command == &"use door" {
                            if self.lever {
                                event_queue.schedule_action(Action::Message(
                                    String::from("You open the door and pass through."),
                                    GameEventType::Success,
                                ));
                                event_queue.schedule_action(Action::Leave(RoomType::WakeUp));
                                event_queue.schedule_action(Action::Enter(RoomType::Locked));
                                true
                            } else {
                                event_queue.schedule_action(Action::Message(
                                        String::from("No matter how hard you push the door, it does not even move an inch."),
                                        GameEventType::Failure,
                                        ));
                                true
                            }
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

// Second room room, locked per default, lever needs to be pulled.
#[derive(Debug)]
pub struct LockedRoom;

impl Room for LockedRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> bool {
        match action {
            Action::Enter(room) => match room {
                RoomType::Locked => {
                    event_queue.schedule_action(Action::Message(
                        String::from(
                            "Success, you have entered the second and final room! You win!",
                        ),
                        GameEventType::Success,
                    ));
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}
