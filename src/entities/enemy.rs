use std::fmt::Debug;

use crate::room::RoomType;
use crate::state::State;
use crate::timer::{Timer, TimerType};
use crate::{Action, GameEventType};

#[derive(Debug, Copy, Clone)]
pub enum EnemyType {
    Rat,
}

pub trait Enemy: Debug {
    fn get_enemy_type(&self) -> EnemyType;

    fn get_health(&self) -> i32;

    fn reduce_health(&mut self, amount: i32) -> bool;

    fn get_attack_strength(&self) -> i32;

    fn get_attack_timers(&self) -> Vec<Timer>;
}

#[derive(Debug)]
pub struct GenericEnemy {
    enemy_type: EnemyType,
    health: i32,
    attack_strength: i32,
    timer_length: u64,
}

impl GenericEnemy {
    pub fn new(
        enemy_type: EnemyType,
        health: i32,
        attack_strength: i32,
        timer_length: u64,
    ) -> Self {
        GenericEnemy {
            enemy_type,
            health,
            attack_strength,
            timer_length,
        }
    }
}

impl Enemy for GenericEnemy {
    fn get_enemy_type(&self) -> EnemyType {
        self.enemy_type
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn reduce_health(&mut self, amount: i32) -> bool {
        self.health -= amount;
        if self.health <= 0 {
            return true;
        }

        false
    }

    fn get_attack_strength(&self) -> i32 {
        self.attack_strength
    }

    fn get_attack_timers(&self) -> Vec<Timer> {
        vec![
            Timer::new(
                TimerType::EnemyAttack,
                // Unused, because invisible.
                &format!("{:?} attack notification timer", self.enemy_type),
                0,
                self.timer_length - self.timer_length / 10,
                Action::Message(
                    String::from(format!("The {:?}'s attack is imminent.", self.enemy_type)),
                    GameEventType::Combat,
                ),
                // Should not be visible as a progressbar.
                false,
            ),
            Timer::new(
                TimerType::EnemyAttack,
                &String::from(format!(
                    "The {:?} is preparing to attack you.",
                    self.enemy_type
                )),
                0,
                self.timer_length,
                Action::EnemyAttack,
                true,
            ),
        ]
    }
}

pub fn initialize_enemies(state: &mut State) {
    let rat = GenericEnemy::new(EnemyType::Rat, 5, 1, 60 * 1000);
    state.enemies.insert(RoomType::SlushLobby, Box::new(rat));
}
