use std::fmt::Debug;

use crate::enemy::Enemy;
use crate::enemy::{EnemyType, GenericEnemy};
use crate::EventQueue;
use crate::{Action, ActionHandled, State};

pub trait Room: Debug {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RoomType {
    Cryobay,
    SlushLobby,
    Cryocontrol,
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
    ) -> ActionHandled {
        ActionHandled::NotHandled
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
    ) -> ActionHandled {
        ActionHandled::NotHandled
    }
}

pub fn room_game_name(room_type: RoomType) -> &'static str {
    match room_type {
        RoomType::Cryobay => "cryobay",
        RoomType::SlushLobby => "slush lobby",
        RoomType::Cryocontrol => "cryocontrol",
    }
}

pub fn room_intro_text(room_type: RoomType) -> &'static str {
    match room_type {
        RoomType::Cryobay => include_str!("../assets/rooms/cryobay_enter.txt"),
        RoomType::SlushLobby => include_str!("../assets/rooms/slush_lobby_enter.txt"),
        RoomType::Cryocontrol => include_str!("../assets/rooms/cryocontrol_enter.txt"),
    }
}

pub fn adjacent_rooms(room_type: RoomType) -> Vec<RoomType> {
    match room_type {
        RoomType::Cryobay => vec![RoomType::SlushLobby],
        RoomType::SlushLobby => vec![RoomType::Cryobay, RoomType::Cryocontrol],
        RoomType::Cryocontrol => vec![RoomType::SlushLobby],
    }
}

pub fn room_type_from_name(room_name: &str) -> Option<RoomType> {
    match room_name {
        "cryobay" => Some(RoomType::Cryobay),
        "slush lobby" => Some(RoomType::SlushLobby),
        "cryocontrol" => Some(RoomType::Cryocontrol),
        _ => None,
    }
}
