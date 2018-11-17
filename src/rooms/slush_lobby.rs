use crate::entities::enemy::{Enemy, EnemyType, GenericEnemy};
use crate::room::{Room, RoomType};
use crate::EventQueue;
use crate::{Action, ActionHandled, State};

// Second room room, locked per default, lever needs to be pulled.
#[derive(Debug)]
pub struct SlushLobbyRoom;

impl SlushLobbyRoom {
    pub fn new() -> SlushLobbyRoom {
        SlushLobbyRoom {}
    }
}

impl Room for SlushLobbyRoom {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled {
        match action {
            _ => return ActionHandled::NotHandled,
        }
    }
}
