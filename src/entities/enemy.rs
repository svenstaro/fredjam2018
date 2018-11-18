extern crate rand;

use rand::Rng;

use std::fmt::Debug;

use crate::room::RoomType;
use crate::state::State;
use crate::timer::{Timer, TimerType};
use crate::{Action, GameEventType};

#[derive(Debug, Copy, Clone)]
pub enum EnemyType {
    Rat,
    Roomba,
}

pub trait Enemy: Debug {
    fn get_enemy_type(&self) -> EnemyType;

    fn get_health(&self) -> i32;

    fn reduce_health(&mut self, amount: i32) -> bool;

    fn get_attack_strength(&self) -> i32;

    fn get_initial_attack_timers(&self, delay: u64) -> Vec<Timer>;

    fn get_attack_timer(&self, delay: u64) -> Timer;

    fn get_attack_message(&self) -> String;

    fn get_enemy_attack_message(&self) -> String;
}

#[derive(Debug)]
pub struct GenericEnemy {
    enemy_type: EnemyType,
    health: i32,
    attack_strength: i32,
    timer_length: u64,
    attack_messages: Vec<String>,
    enemy_attack_messages: Vec<String>,
}

impl GenericEnemy {
    pub fn new(
        enemy_type: EnemyType,
        health: i32,
        attack_strength: i32,
        timer_length: u64,
        attack_messages: Vec<String>,
        enemy_attack_messages: Vec<String>,
    ) -> Self {
        GenericEnemy {
            enemy_type,
            health,
            attack_strength,
            timer_length,
            attack_messages,
            enemy_attack_messages,
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

    fn get_attack_message(&self) -> String {
        if let Some(message) = rand::thread_rng().choose(&self.attack_messages) {
            message.to_string()
        } else {
            String::from(format!("You attack the {:?}.", self.enemy_type))
        }
    }

    fn get_enemy_attack_message(&self) -> String {
        if let Some(message) = rand::thread_rng().choose(&self.enemy_attack_messages) {
            message.to_string()
        } else {
            String::from(format!("The {:?} attacks!", self.enemy_type))
        }
    }

    fn get_initial_attack_timers(&self, delay: u64) -> Vec<Timer> {
        vec![
            Timer::new(
                TimerType::EnemyAttack,
                // Unused, because invisible.
                &format!("{:?} attack notification timer", self.enemy_type),
                0,
                delay,
                Action::Message(
                    String::from(format!("The {:?}'s attack is imminent.", self.enemy_type)),
                    GameEventType::Combat,
                ),
                // Should not be visible as a progressbar.
                false,
            ),
            self.get_attack_timer(delay),
        ]
    }

    fn get_attack_timer(&self, delay: u64) -> Timer {
        let show_bar = (delay <= 0);
        Timer::new(
            TimerType::EnemyAttack,
            &format!("The {:?} is preparing to attack you.", self.enemy_type),
            0,
            self.timer_length + delay,
            Action::EnemyAttack,
            show_bar,
        )
    }
}

pub fn initialize_enemies(state: &mut State) {
    let rat_attack_messages = vec!["You stomp on the rat.".into()];
    let rat_enemy_attack_messages = vec!["The rat gnaws on your leg.".into()];

    let rat = GenericEnemy::new(
        EnemyType::Rat,
        5,
        1,
        5 * 1000,
        rat_attack_messages,
        rat_enemy_attack_messages,
    );
    state.enemies.insert(RoomType::Cryocontrol, Box::new(rat));

    let roomba_attack_messages = vec!["".into()];
    let roomba_enemy_attack_messages = vec![];

    let roomba = GenericEnemy::new(
        EnemyType::Roomba,
        5,
        1,
        5 * 1000,
        roomba_attack_messages,
        roomba_enemy_attack_messages,
    );
    state.enemies.insert(RoomType::Corridor, Box::new(roomba));
}
