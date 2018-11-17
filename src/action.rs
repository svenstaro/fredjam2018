use crate::rooms::RoomType;
use crate::game_event::GameEventType;

// TODO Extend this to have timers (if needed?)
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Action {
    // String is room name.
    Enter(RoomType),
    Tick(u32),
    // String is room name.
    Leave(RoomType),
    Message(String, GameEventType),
    Command(String),
}
