use std::fmt::Debug;

use crate::enemy::Enemy;
use crate::EventQueue;
use crate::{Action, GameEventType, State};
use crate::enemy::{EnemyType, GenericEnemy};

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
    Cryobay,
    SlushLobby,
}

// Initial room.
#[derive(Debug)]
pub struct CryobayRoom {
    pub lever: bool,
}

impl CryobayRoom {
    pub fn new() -> CryobayRoom {
        CryobayRoom { lever: false }
    }
}

impl Room for CryobayRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> bool {
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
                                event_queue.schedule_action(Action::Leave(RoomType::Cryobay));
                                event_queue.schedule_action(Action::Enter(RoomType::SlushLobby));
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
pub struct SlushLobbyRoom;

impl SlushLobbyRoom {
    pub fn new() -> SlushLobbyRoom {
        SlushLobbyRoom {}
    }
}

impl Room for SlushLobbyRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> bool {
        match action {
            Action::Enter(room_type) => {
                match room_type {
                    RoomType::SlushLobby => {
                        let rat = GenericEnemy::new(EnemyType::Rat, 5, 1, 60*1000);
                        let timer = rat.get_attack_timer();
                        event_queue.schedule_timer(timer);
                        state.enemy = Some(Box::new(rat));
                    }
                    _ => {}
                }
            }
            _ => {},
        }
        false
    }
}

pub fn room_game_name(room_type: RoomType) -> &'static str {
    match room_type {
        RoomType::Cryobay => "cryobay",
        RoomType::SlushLobby => "slush lobby",
    }
}

pub fn room_intro_text(room_type: RoomType) -> &'static str {
    match room_type {
        RoomType::Cryobay => include_str!("../assets/rooms/cryobay_enter.txt"),
        RoomType::SlushLobby => include_str!("../assets/rooms/slush_lobby_enter.txt"),
    }
}
