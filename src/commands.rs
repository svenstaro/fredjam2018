use crate::action::Action;
use crate::game_event::GameEventType;
use crate::room::{adjacent_rooms, room_type_from_name};
use crate::state::State;

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
        _ => vec![Action::Message(
            String::from("Use one of the following commands: enter, attack, dodge."),
            GameEventType::Failure,
        )],
        Some("dodge") => vec![Action::Dodge],
        _ => vec![Action::Message(
            String::from("Use one of the following commands: enter, attack, dodge."),
            GameEventType::Failure,
        )],
        Some("pick up crowbar") => vec![Action::PickUpCrowbar],
        Some("pick up keycard") => vec![Action::PickUpKeycard],
        Some("use keycard") => vec![Action::UseKeycard],
        Some("use crowbar") => vec![Action::UseCrowbar],
        Some("use casket") => vec![Action::UseCrowbar],
        Some("use lever") => vec![Action::UseLever],
        Some("use door") => vec![Action::UseDoor],

        Some("look around") => vec![Action::LookAround],
        _ => vec![Action::Message(
            String::from("Use one of the following commands: enter, attack, dodge."),
            GameEventType::Failure,
        )],
    }
}
