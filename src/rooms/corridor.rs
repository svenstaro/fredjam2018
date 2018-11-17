use crate::entities::enemy::{EnemyType, GenericEnemy};
use crate::room::{Room, RoomType};
use crate::EventQueue;
use crate::{Action, ActionHandled, State};

#[derive(Debug)]
pub struct CorridorRoom {
    pub visited: bool,
}

impl CorridorRoom {
    pub fn new() -> CorridorRoom {
        CorridorRoom { visited: false }
    }
}

impl Room for CorridorRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled {
        ActionHandled::NotHandled
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }
}
