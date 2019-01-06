use crate::action::Action;
use crate::game_event::GameEventType;
use crate::room::{adjacent_rooms, room_type_from_name};
use crate::state::State;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "command.pest"]
struct CommandParser;

static HELP_TEXT: &'static str =
    "Use one of the following commands: enter, attack, dodge, pickup, use.";

pub fn try_handle_command(tokens: String, state: &State) -> Vec<Action> {
    let mut parse = match CommandParser::parse(Rule::command, &tokens) {
        Ok(parse) => parse.flatten(),
        Err(_) => {
            return vec![Action::Message(HELP_TEXT.into(), GameEventType::Failure)];
        }
    };
    let object = parse.clone().find(|pair| pair.as_rule() == Rule::object)
        .map(|v| v.as_str());
    let verb = parse.find(|pair| pair.as_rule() == Rule::verb);

    match verb.map(|v| v.as_str()) {
        Some("enter") => {
            if let Some(room_name) = object {
                let available_rooms = adjacent_rooms(state.current_room);
                let maybe_room_type = room_type_from_name(&room_name);
                match maybe_room_type {
                    Some(room_type) => {
                        if available_rooms.contains(&room_type) || cfg!(debug_assertions) {
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
            } else {
                vec![Action::Message(
                    String::from("Specify a room to enter!"),
                    GameEventType::Failure,
                )]
            }
        }
        Some("look") => vec![Action::ShowEnterText],
        Some("attack") => vec![Action::Attack],
        Some("dodge") => vec![Action::Dodge],
        Some("pickup") => {
            match object {
                Some("crowbar") => vec![Action::PickUpCrowbar],
                Some("keycard") => vec![Action::PickUpKeycard],
                _ => vec![Action::Message(
                    String::from("No such item."),
                    GameEventType::Failure,
                )],
            }
        }
        Some("use") => {
            match object {
                Some("crowbar") => vec![Action::UseCrowbar],
                Some("terminal") => vec![Action::UseTerminal],
                Some("keycard") => vec![Action::UseKeycard],
                Some("casket") => vec![Action::UseCasket],
                Some("door") => vec![Action::UseDoor],
                _ => vec![Action::Message(
                    String::from("No such item."),
                    GameEventType::Failure,
                )],
            }
        }
        _ => vec![Action::Message(HELP_TEXT.into(), GameEventType::Failure)],
    }
}
