extern crate rand;

use rand::Rng;

use std::fmt::Debug;

use crate::room::RoomType;
use crate::sound::{AudioEvent, Effect};
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

    fn get_initial_attack_timers(&self) -> Vec<Timer>;

    fn get_attack_timers(&self, delay: u64) -> Vec<Timer>;

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

    fn get_initial_attack_timers(&self) -> Vec<Timer> {
        self.get_attack_timers(2000)
    }

    fn get_attack_timers(&self, delay: u64) -> Vec<Timer> {
        vec![
            Timer::new(
                TimerType::EnemyAttack,
                &format!("The {:?} is preparing to attack you.", self.enemy_type),
                0,
                self.timer_length + delay,
                Action::EnemyAttack,
                true,
            ),
            Timer::new(
                TimerType::EnemyAttack,
                // Unused, because invisible.
                "",
                0,
                self.timer_length + delay,
                Action::Audio(AudioEvent::Effect(Effect::EnemyAttack)),
                // Should not be visible as a progressbar.
                false,
            ),
        ]
    }
}

pub fn initialize_enemies(state: &mut State) {
    let rat_attack_messages = vec!["You stomp on the rat.".into()];
    let rat_enemy_attack_messages = vec![
        "The rat gnaws on your leg.".into(),
        "The rat runs around you in circles. You try to follow it, stumbling.".into(),
    ];

    let rat = GenericEnemy::new(
        EnemyType::Rat,
        10,
        5,
        8 * 1000,
        rat_attack_messages,
        rat_enemy_attack_messages,
    );
    state.enemies.insert(RoomType::Corridor, Box::new(rat));

    let roomba_attack_messages = vec![
        "You tackle the roomba. It topples over.".into(),
        "You smash in one of the roombas many visual sensors.".into(),
        "You kick the roomba, leaving a dent.".into(),
        "You rip out one of the roombas appendages. It produces a high-pitched beeping wail."
            .into(),
    ];
    let roomba_enemy_attack_messages = vec![
        "The roomba vacuums your arm. Some of the skin comes off.".into(),
        "The roomba swings its broom and hits your head.".into(),
    ];

    let roomba = GenericEnemy::new(
        EnemyType::Roomba,
        40,
        20,
        3 * 1000,
        roomba_attack_messages,
        roomba_enemy_attack_messages,
    );
    state
        .enemies
        .insert(RoomType::Cryocontrol, Box::new(roomba));
}
