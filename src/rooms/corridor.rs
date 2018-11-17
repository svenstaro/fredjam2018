use crate::EventQueue;
use crate::{Action, ActionHandled, State};
use crate::room::{Room, RoomType};
use crate::entities::enemy::{EnemyType, GenericEnemy};

#[derive(Debug)]
pub struct CorridorRoom {}

impl CorridorRoom {
    pub fn new() -> CorridorRoom {
        CorridorRoom {}
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
}


