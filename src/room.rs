use std::fmt::Debug;

use strum::EnumProperty;

use crate::game_event::{GameEvent, GameEventType};
use crate::sound::{AudioEvent, Track};
use crate::App;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumProperty)]
pub enum RoomType {
    #[strum(props(game_name = "cryobay"))]
    Cryobay,
    #[strum(props(game_name = "slush lobby"))]
    SlushLobby,
    #[strum(props(game_name = "cryocontrol"))]
    Cryocontrol,
    #[strum(props(game_name = "ventilation shaft"))]
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

pub fn closed_message(room_type: RoomType) -> String {
    match room_type {
        RoomType::Corridor => "Peering through the ventilation shafts, \
                               it looks like they connect to a corridor. \
                               You would need a tool to get through."
            .into(),
        _ => format!(
            "The door to {} is closed and won't open.",
            room_type.get_str("game_name").unwrap()
        )
        .into(),
    }
}

fn change_music(app: &mut App, room_type: RoomType) {
    app.event_queue
        .schedule_action(Action::Audio(AudioEvent::Track(room_type.get_track())));
}

pub fn enter_room(app: &mut App, room_type: RoomType) {
    change_music(app, room_type);

    app.state.current_room = room_type;
    let available_rooms = adjacent_rooms(room_type);
    let plural = if available_rooms.len() > 1 { "s" } else { "" };
    let mut door_msg = format!(
        "\nYou see {} door{} labeled:\n",
        &available_rooms.len(),
        plural
    );
    for room in available_rooms {
        door_msg += "  - ";
        door_msg += room.get_str("game_name").unwrap();
        door_msg += "\n";
    }

    app.log.push_front(GameEvent {
        content: door_msg,
        game_event_type: GameEventType::Normal,
    });

    app.event_queue
        .schedule_actions(room_specific_actions(&app, room_type));

    let has_visited = app.rooms.get(&room_type).unwrap().is_visited();
    if has_visited {
        app.event_queue.schedule_action(Action::Message(
            String::from(room_intro_text(room_type).0),
            GameEventType::Normal,
        ));
    } else {
        app.event_queue.schedule_action(Action::Message(
            String::from(room_intro_text(room_type).1),
            GameEventType::Normal,
        ));
    }
    app.rooms.get_mut(&room_type).unwrap().visit();
}

fn room_specific_actions(app: &App, room_type: RoomType) -> Vec<Action> {
    match room_type {
        RoomType::Cryocontrol => {
            if app.rooms.get(&room_type).unwrap().is_visited()
                && app.state.get_current_enemy(room_type).is_none()
            {
                vec![Action::Message(
                    "The central cortex rumbles uneasily. There's a terminal in front of it."
                        .into(),
                    GameEventType::Normal,
                )]
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}
