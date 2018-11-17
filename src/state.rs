use crate::rooms::RoomType;

use crate::enemy::Enemy;
use crate::player::Player;

#[derive(Debug)]
pub struct State {
    pub current_room: RoomType,
    pub player: Player,
    pub enemy: Option<Box<Enemy>>,
}

impl State {
    pub fn new() -> Self {
        State {
            current_room: RoomType::Cryobay,
            player: Player {
                health: 100,
                items: vec![],
            },
            enemy: None,
        }
    }
}
