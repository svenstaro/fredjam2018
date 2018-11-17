use std::collections::HashMap;
use std::fmt::Debug;

use crate::timer::Timer;
use crate::{Action, GameEventType};

#[derive(Debug)]
pub enum EnemyType {
    Rat,
}

pub trait Enemy: Debug {
    fn get_enemy_type(self) -> EnemyType;

    fn get_health(self) -> i32;

    fn reduce_health(&mut self, amount: i32) -> bool;

    fn get_attack_timer(self) -> Timer;
}

#[derive(Debug)]
pub struct GenericEnemy {
    enemy_type: EnemyType,
    health: i32,
    attack_strength: i32,
    timer_length: u64,
}

impl Enemy for GenericEnemy {
    fn get_enemy_type(self) -> EnemyType {
        self.enemy_type
    }

    fn get_health(self) -> i32 {
        self.health
    }

    fn reduce_health(&mut self, amount: i32) -> bool {
        self.health -= amount;
        if self.health <= 0 {
            return true;
        }

        false
    }

    fn get_attack_timer(self) -> Timer {
        let mut timings = HashMap::new();
        timings.insert(
            10,
            Action::Message(
                String::from(format!("The {:?} prepares to attack you.", self.enemy_type)),
                GameEventType::Success,
            ),
        );
        timings.insert(
            10,
            Action::Message(
                String::from(format!("The {:?}'s attack is imminent", self.enemy_type)),
                GameEventType::Success,
            ),
        );

        Timer::new(
            &format!("{:?} attack timer", self.enemy_type),
            self.timer_length,
            0,
            timings,
        )
    }
}
