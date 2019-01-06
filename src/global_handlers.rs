use crate::action::{Action, ActionHandled};
use crate::game_event::{GameEvent, GameEventType};
use crate::room::{closed_message, enter_room, RoomType};
use crate::sound::{AudioEvent, Effect};
use crate::timer::TimerType;
use crate::App;
use crate::room;

// Handle game actions here (Timers).
pub fn handle_action(mut app: &mut App, next_action: Action) {
    if app.try_handle_room_action(&next_action).is_some() {
        return
    }

    // Handle system and global actions here.
    match next_action {
        Action::Message(mut message, game_event_type) => {
            message.push('\n');
            app.log.push_front(GameEvent {
                content: message,
                game_event_type: game_event_type,
            })
        }
        Action::Rebooted => {
            app.event_queue.schedule_action(Action::Message(
                String::from("\"Initiating reboot.\" The room goes black. You notice a coldness quickly creeping into the air and start to shiver. As the air gets thinner and thinner, you slowly slip into unconsciousness."),
                GameEventType::Failure,
            ));
        }
        Action::Enter(room_type) => {
            if app.rooms.get(&room_type).unwrap().is_opened() || cfg!(debug_assertions) {
                if room_type != RoomType::Corridor {
                    app.event_queue
                        .schedule_action(Action::Audio(AudioEvent::Effect(Effect::Door)));
                }
                enter_room(&mut app, room_type);
            } else {
                app.event_queue.schedule_action(Action::Message(
                    closed_message(room_type),
                    GameEventType::Failure,
                ));
            }
        }
        Action::Leave(_) => {}
        Action::ShowEnterText => {
            app.event_queue.schedule_action(Action::Message(
                room::room_intro_text(app.state.current_room).0.into(),
                GameEventType::Normal,
            ));
        }
        Action::Command(tokens) => app.try_handle_command(tokens),
        Action::EnemyAttack => {
            if let Some(ref enemy) = app.state.get_current_enemy(app.state.current_room) {
                let timers = enemy.get_attack_timers(0);
                for timer in timers {
                    app.event_queue.schedule_timer(timer);
                }

                app.log.push_front(GameEvent {
                    content: format!(
                        "{} You lose {} HP.\n",
                        enemy.get_enemy_attack_message(),
                        enemy.get_attack_strength(),
                    ),
                    game_event_type: GameEventType::Combat,
                });

                app.state.player.health -= enemy.get_attack_strength();
                if app.state.player.health <= 0 {
                    app.event_queue.schedule_action(Action::PlayerDied);
                }
            }
        }
        Action::Attack => {
            let damage = app.state.player.attack_strength;
            if let Some(ref mut enemy) = app.state.get_current_enemy_mut(app.state.current_room) {
                enemy.reduce_health(damage);
                let attack_message = enemy.get_attack_message();
                let enemy_type = enemy.get_enemy_type();
                if enemy.get_health() <= 0 {
                    app.event_queue
                        .schedule_action(Action::Audio(AudioEvent::Effect(
                            Effect::PlayerAttack,
                        )));
                    app.log.push_front(GameEvent {
                        content: enemy.get_death_message(),
                        game_event_type: GameEventType::Failure,
                    });
                    app.event_queue
                        .emplace_timers(TimerType::EnemyAttack, vec![]);
                    app.state.enemies.remove(&app.state.current_room);
                }
                app.log.push_front(GameEvent {
                    content: format!("{}\n", attack_message),
                    game_event_type: GameEventType::Combat,
                });
            } else {
                app.event_queue.schedule_action(Action::Message(
                    String::from("There is nothing you can attack."),
                    GameEventType::Failure,
                ));
            }
        }
        Action::PlayerDied => {
            app.event_queue.schedule_action(Action::Message(
                String::from("You died."),
                GameEventType::Failure,
            ));
        }
        Action::Dodge => {
            let mut attack_timers = app.event_queue.get_timers(TimerType::EnemyAttack);
            if attack_timers.is_empty() {
                if app.state.enemies.get(&app.state.current_room).is_some() {
                    app.event_queue.schedule_action(Action::Message(
                        String::from(
                            "You dodge the attack. The enemy calmly analyses your movements.",
                        ),
                        GameEventType::Failure,
                    ));
                    return;
                }
                app.event_queue.schedule_action(Action::Message(
                    String::from("You dodge the attack of your own paranoia..."),
                    GameEventType::Failure,
                ));
                return;
            }

            for elem in attack_timers.iter_mut() {
                elem.elapsed = 0;
            }
            if let Some(enemy) = app.state.get_current_enemy(app.state.current_room) {
                app.event_queue
                    .emplace_timers(TimerType::EnemyAttack, enemy.get_attack_timers(0));
            }

            let enemy_type = app
                .state
                .enemies
                .get(&app.state.current_room)
                .unwrap()
                .get_enemy_type();
            app.event_queue.schedule_action(Action::Message(
                String::from(format!("You dodge the {:?}'s attack.", enemy_type)),
                GameEventType::Success,
            ));
        }
        Action::Tick(dt) => {
            app.event_queue.tick(dt);
        }
        Action::PlayerFinishedReading => {
            let room_type = app.state.current_room;
            let enemy_option = app.state.get_current_enemy(room_type);

            let timers = app.event_queue.get_timers(TimerType::EnemyAttack);
            if !timers.is_empty() {
                return;
            }

            match enemy_option {
                Some(enemy) => {
                    let timers = enemy.get_initial_attack_timers();
                    app.event_queue.schedule_timers(timers);
                    app.log.push_front(GameEvent {
                        content: format!("The {:?}'s attack is imminent.\n", enemy.get_enemy_type()),
                        game_event_type: GameEventType::Combat,
                    });
                }
                None => (),
            }
        }
        action => app.log.push_front(GameEvent {
            content: format!("Unhandled action: {}\n", action),
            game_event_type: GameEventType::Debug,
        }),
    }
}
