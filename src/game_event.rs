#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GameEventType {
    Combat,
    Normal,
    Success,
    Failure,
    Debug,
}

#[derive(Debug)]
pub struct GameEvent {
    pub content: String,
    pub game_event_type: GameEventType,
}
