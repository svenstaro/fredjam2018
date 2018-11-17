use std::fmt::Debug;

use crate::App;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};
use crate::game_event::GameEventType;

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

pub fn enter_room(app: &mut App, room_type: RoomType) {
    let enemy_option = app.state.get_current_enemy(room_type);
    match enemy_option {
        Some(enemy) => {
            let timers = enemy.get_attack_timers();
            app.event_queue.schedule_timers(timers);
        },
        None => (),
    }

    app.state.current_room = room_type;
    let available_rooms = adjacent_rooms(room_type);
    let mut door_msg = String::from("\n\nYou see ")
        + &available_rooms.len().to_string()
        + " doors labeled:\n";
    for room in available_rooms {
        door_msg += "  - ";
        door_msg += room_game_name(room);
        door_msg += "\n";
    }
    app.event_queue.schedule_action(Action::Message(
            String::from(room_intro_text(room_type).to_owned() + &door_msg),
            GameEventType::Normal,
            ));
}
