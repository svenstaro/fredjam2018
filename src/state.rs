use crate::rooms::RoomType;

#[derive(Debug)]
pub struct State {
    pub current_room: RoomType,
}

impl State {
    pub fn new() -> Self {
        State {
            current_room: RoomType::WakeUp,
        }
    }
}
