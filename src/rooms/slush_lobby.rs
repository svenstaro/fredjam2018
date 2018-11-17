use crate::EventQueue;
use crate::{Action, ActionHandled, State};
use crate::room::{Room, RoomType};
use crate::entities::enemy::{EnemyType, GenericEnemy, Enemy};

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
            Action::Enter(RoomType::SlushLobby) => {
                let rat = GenericEnemy::new(EnemyType::Rat, 5, 1, 5 * 1000);
                let timers = rat.get_attack_timers();
                for timer in timers {
                    event_queue.schedule_timer(timer);
                }
                state.enemies.insert(RoomType::Cryobay, Box::new(rat));

                ActionHandled::NotHandled
            },
            _ => return ActionHandled::NotHandled,
        }
    }
}


