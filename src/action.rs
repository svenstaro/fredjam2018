use crate::game_event::GameEventType;
use crate::rooms::RoomType;

// TODO Extend this to have timers (if needed?)
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Action {
    // String is room name.
    Enter(RoomType),
    Tick(u64),
    // String is room name.
    Leave(RoomType),
    Message(String, GameEventType),
    Command(String),

    // Player
    Attack(String),
    Dodge(String),
    PlayerDied,

    // Enemy attack
    EnemyAttack,
}
