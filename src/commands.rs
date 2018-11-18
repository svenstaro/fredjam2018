use crate::action::Action;
use crate::game_event::GameEventType;
use crate::room::{adjacent_rooms, room_type_from_name};
use crate::state::State;

static HELP_TEXT: &'static str = "Use one of the following commands: enter, attack, dodge, pickup, use.";

pub fn try_handle_command(tokens: String, state: &State) -> Vec<Action> {
    let mut split = tokens.split(" ");
    let command = split.next();
    match command {
        Some("enter") => {
            let available_rooms = adjacent_rooms(state.current_room);
            let room_name_parts: Vec<&str> = split.collect();
            let room_name = room_name_parts.join(" ");
            let maybe_room_type = room_type_from_name(&room_name);
            match maybe_room_type {
                Some(room_type) => {
                    if available_rooms.contains(&room_type) {
                        vec![Action::Leave(state.current_room), Action::Enter(room_type)]
                    } else {
                        vec![Action::Message(
                            String::from("You can't go to ") + &room_name + " from here.",
                            GameEventType::Failure,
                        )]
                    }
                }
                None => vec![Action::Message(
                    String::from("There is no room named \"") + &room_name + "\".",
                    GameEventType::Failure,
                )],
            }
        }
        Some("attack") => vec![Action::Attack],
        Some("dodge") => vec![Action::Dodge],
        Some("pickup") => {
            let item = split.next();
            match item {
                Some("crowbar") => vec![Action::PickUpCrowbar],
                Some("keycard") => vec![Action::PickUpKeycard],
                _ => vec![Action::Message(
                    String::from("No such item."),
                    GameEventType::Failure,
                    )]
            }
        }
        Some("use") => {
            let item = split.next();
            match item {
                Some("crowbar") => vec![Action::UseCrowbar],
                Some("terminal") => vec![Action::UseTerminal],
                Some("keycard") => vec![Action::UseKeycard],
                Some("casket") => vec![Action::UseCrowbar],
                Some("lever") => vec![Action::UseLever],
                Some("door") => vec![Action::UseDoor],
                _ => vec![Action::Message(
                    String::from("No such item."),
                    GameEventType::Failure,
                    )]
            }
        }
        _ => vec![Action::Message(
            HELP_TEXT.into(),
            GameEventType::Failure,
        )],
    }
}

