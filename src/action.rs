use crate::game_event::GameEventType;
use crate::rooms::RoomType;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
