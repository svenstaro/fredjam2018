use crate::entities::Item;
use crate::game_event::GameEventType;
use crate::room::RoomType;
use crate::sound::AudioEvent;

pub enum ActionHandled {
    Handled,
    NotHandled,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Display)]
pub enum Action {
    // System Actions.
    Tick(u64),
    Message(String, GameEventType),
    Command(String),
    PlayerFinishedReading,
    GameOver,

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

    //Pickup
    PickUpKeycard,
    PickUpCrowbar,

    // Game logic actions
    PlayerDied,

    UseDoor,
    UseKeycard,
    UseCrowbar,
    UseCasket,
    UseTerminal,

    // Open Rooms
    OpenCorridor,
    OpenCryoControl,

    ShowEnterText,

    Rebooted,
}
