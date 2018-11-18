use std::fmt::Debug;

use crate::game_event::GameEventType;
use crate::App;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};
use crate::sound::{AudioEvent, Track};

pub trait Room: Debug {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled;

    fn is_opened(&self) -> bool;
    fn open(&mut self);
    fn visit(&mut self);
    fn is_visited(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RoomType {
    Cryobay,
    SlushLobby,
    Cryocontrol,
    Corridor,
}

impl RoomType {
    fn get_track(&self) -> Track {
        match self {
            RoomType::Cryobay => Track::Intro,
            RoomType::Cryocontrol => Track::Complications,
            _ => Track::Loop,
        }
    }
}

pub fn room_game_name(room_type: RoomType) -> &'static str {
    match room_type {
        RoomType::Cryobay => "cryobay",
        RoomType::SlushLobby => "slush lobby",
        RoomType::Cryocontrol => "cryocontrol",
        RoomType::Corridor => "ventilation shaft",
    }
}

pub fn room_intro_text(room_type: RoomType) -> (&'static str, &'static str) {
    match room_type {
        RoomType::Cryobay => (
            include_str!("../assets/rooms/cryobay_enter.txt"),
            include_str!("../assets/rooms/cryobay_enter_first.txt"),
        ),
        RoomType::SlushLobby => (
            include_str!("../assets/rooms/slush_lobby_enter.txt"),
            include_str!("../assets/rooms/slush_lobby_enter_first.txt"),
        ),
        RoomType::Cryocontrol => (
            include_str!("../assets/rooms/cryocontrol_enter.txt"),
            include_str!("../assets/rooms/cryocontrol_enter_first.txt"),
        ),
        RoomType::Corridor => (
            include_str!("../assets/rooms/corridor_enter.txt"),
            include_str!("../assets/rooms/corridor_enter_first.txt"),
        ),
    }
}

pub fn reading_time_msecs(room_type: RoomType, has_visited: bool) -> u64 {
    let intros = room_intro_text(room_type);
    let msg = if has_visited { intros.0 } else { intros.1 };
    // 0.05 of a second for every character, times 1000 to convert to msecs
    (msg.len() as f64 * 0.05 * 1000.0).floor() as u64
}

pub fn adjacent_rooms(room_type: RoomType) -> Vec<RoomType> {
    match room_type {
        RoomType::Cryobay => vec![RoomType::SlushLobby],
        RoomType::SlushLobby => vec![RoomType::Cryobay, RoomType::Cryocontrol, RoomType::Corridor],
        RoomType::Cryocontrol => vec![RoomType::SlushLobby],
        RoomType::Corridor => vec![RoomType::SlushLobby],
    }
}

pub fn room_type_from_name(room_name: &str) -> Option<RoomType> {
    match room_name {
        "cryobay" => Some(RoomType::Cryobay),
        "slush lobby" => Some(RoomType::SlushLobby),
        "cryocontrol" => Some(RoomType::Cryocontrol),
        "ventilation shaft" => Some(RoomType::Corridor),
        _ => None,
    }
}

fn change_music(app: &mut App, room_type: RoomType) {
    app.event_queue.schedule_action(Action::Audio(
        AudioEvent::Track(room_type.get_track())
    ));
}

pub fn enter_room(app: &mut App, room_type: RoomType) {
    let enemy_option = app.state.get_current_enemy(room_type);
    let has_visited = app.rooms.get(&room_type).unwrap().is_visited();
    match enemy_option {
        Some(enemy) => {
            let timers = enemy.get_initial_attack_timers(reading_time_msecs(room_type, has_visited));
            app.event_queue.schedule_timers(timers);
        }
        None => (),
    }
    change_music(app, room_type);

    app.state.current_room = room_type;
    let available_rooms = adjacent_rooms(room_type);
    let plural = if available_rooms.len() > 1 { "s" } else { "" };
    let mut door_msg =
        format!("\nYou see {} door{} labeled:\n", &available_rooms.len(), plural);
    for room in available_rooms {
        door_msg += "  - ";
        door_msg += room_game_name(room);
        door_msg += "\n";
    }
    if has_visited {
        app.event_queue.schedule_action(Action::Message(
            String::from(room_intro_text(room_type).0.to_owned() + &door_msg),
            GameEventType::Normal,
        ));
    } else {
        app.event_queue.schedule_action(Action::Message(
            String::from(room_intro_text(room_type).1.to_owned() + &door_msg),
            GameEventType::Normal,
        ));
    }
    app.rooms.get_mut(&room_type).unwrap().visit();
}
