use crate::game_event::GameEventType;
use crate::room::RoomType;

pub enum ActionHandled {
    Handled,
    NotHandled,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Action {
    // Do nothing.
    Nop,
    Enter(RoomType),
    Tick(u64),
    Leave(RoomType),
    Message(String, GameEventType),
    Command(String),

    // Player
    Attack,
    Dodge,
    PlayerDied,

    // Enemy attack
    EnemyAttack,
}
