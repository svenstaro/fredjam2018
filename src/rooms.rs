use crate::{Action, GameEventType, State};

pub trait Room {
    fn handle_action(&mut self, state: &mut State, action: &Action) -> bool;
}

// Initial room.
pub struct WakeUpRoom {
    pub lever: bool,
}

impl Room for WakeUpRoom {
    fn handle_action(&mut self, state: &mut State, action: &Action) -> bool {
        match action {
            Action::Enter(room) => {
                if room == &"WakeUp" {
                    state.schedule_action(Action::Message(
                        String::from("Welcome to W.O.R.L.D."),
                        GameEventType::Success,
                    ));
                    true
                } else {
                    false
                }
            }
            Action::Command(command) => {
                // TODO Maybe there's a better approach to finding the current room...
                if state.current_room == "WakeUp" {
                    // TODO replace command by proper enum.
                    if command == &"use lever" {
                        if !self.lever {
                            state.schedule_action(Action::Message(
                                String::from(
                                    "You pull the lever, it flips down with a screeching noize.",
                                ),
                                GameEventType::Success,
                            ));
                            self.lever = true;
                        } else {
                            state.schedule_action(Action::Message(
                                String::from("There is no point in pulling this back up."),
                                GameEventType::Failure,
                            ));
                        }
                        true
                    } else if command == &"look around" {
                        state.schedule_action(Action::Message(
                            String::from("The room is empty, there is just some lever and a solid door with no handle."),
                            GameEventType::Normal,
                        ));
                        true
                    } else if command == &"use door" {
                        if self.lever {
                            state.schedule_action(Action::Message(
                                String::from("You open the door and pass through."),
                                GameEventType::Success,
                            ));
                            state.schedule_action(Action::Leave("WakeUp"));
                            state.schedule_action(Action::Enter("Locked"));
                            true
                        } else {
                            state.schedule_action(Action::Message(
                                String::from("No matter how hard you push the door, it does not even move an inch."),
                                GameEventType::Failure,
                            ));
                            true
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

// Second room room, locked per default, lever needs to be pulled.
pub struct LockedRoom;

impl Room for LockedRoom {
    fn handle_action(&mut self, state: &mut State, action: &Action) -> bool {
        match action {
            Action::Enter(room) => {
                if room == &"Locked" {
                    state.schedule_action(Action::Message(
                        String::from(
                            "Success, you have entered the second and final room! You win!",
                        ),
                        GameEventType::Success,
                    ));
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
