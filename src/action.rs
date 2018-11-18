use crate::game_event::GameEventType;
use crate::room::RoomType;
use crate::sound::AudioEvent;
use crate::entities::Item;

pub enum ActionHandled {
    Handled,
    NotHandled,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Action {
    // System Actions.
    Tick(u64),
    Message(String, GameEventType),
    Command(String),

    // Player
    Attack,
    Dodge,
    PickUp(Item),
    Enter(RoomType),
    Leave(RoomType),

    // Enemy attack
    EnemyAttack,

    // Audio things
    Audio(AudioEvent),

    // Game logic actions
    PlayerDied,
    UseKeycard,
    UseCasket,
    UseTerminal,
}
