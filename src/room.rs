use std::fmt::Debug;

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
    Corridor,
}

pub fn room_game_name(room_type: RoomType) -> &'static str {
    match room_type {
        RoomType::Cryobay => "cryobay",
        RoomType::SlushLobby => "slush lobby",
        RoomType::Cryocontrol => "cryocontrol",
        RoomType::Corridor => "corridor",
    }
}

pub fn room_intro_text(room_type: RoomType) -> &'static str {
    match room_type {
        RoomType::Cryobay => include_str!("../assets/rooms/cryobay_enter.txt"),
        RoomType::SlushLobby => include_str!("../assets/rooms/slush_lobby_enter.txt"),
        RoomType::Cryocontrol => include_str!("../assets/rooms/cryocontrol_enter.txt"),
        RoomType::Corridor => include_str!("../assets/rooms/corridor_enter.txt"),
    }
}

pub fn adjacent_rooms(room_type: RoomType) -> Vec<RoomType> {
    match room_type {
        RoomType::Cryobay => vec![RoomType::SlushLobby],
        RoomType::SlushLobby => vec![RoomType::Cryobay, RoomType::Cryocontrol],
        RoomType::Cryocontrol => vec![RoomType::SlushLobby],
        RoomType::Corridor => vec![RoomType::Cryocontrol],
    }
}

pub fn room_type_from_name(room_name: &str) -> Option<RoomType> {
    match room_name {
        "cryobay" => Some(RoomType::Cryobay),
        "slush lobby" => Some(RoomType::SlushLobby),
        "cryocontrol" => Some(RoomType::Cryocontrol),
        "corridor" => Some(RoomType::Corridor),
        _ => None,
    }
}
