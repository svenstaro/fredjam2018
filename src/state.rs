use std::collections::HashMap;

use crate::enemy::Enemy;
use crate::player::Player;
use crate::rooms::RoomType;

#[derive(Debug)]
pub struct State {
    pub current_room: RoomType,
    pub player: Player,
    pub enemies: HashMap<RoomType, Box<Enemy>>,
}

impl State {
    pub fn new() -> Self {
        State {
            current_room: RoomType::Cryobay,
            player: Player {
                health: 100,
                attack_strength: 5,
                items: vec![],
            },
            enemies: HashMap::new(),
        }
    }

    pub fn get_current_enemy(&mut self, room_type: RoomType) -> Option<&mut Box<Enemy>> {
        self.enemies.get_mut(&room_type)
    }
}
