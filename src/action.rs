use crate::game_event::GameEventType;
use crate::room::RoomType;
use crate::sound::AudioEvent;

pub enum ActionHandled {
    Handled,
    NotHandled,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Action {
    // Do nothing.
    Nop,
    // String is room name.
    Enter(RoomType),
    Tick(u64),
    // String is room name.
    Leave(RoomType),
    Message(String, GameEventType),
    Command(String),

    // Player
    Attack,
    Dodge,
    PlayerDied,

    // Enemy attack
    EnemyAttack,

    // Audio things
    Audio(AudioEvent),
}
