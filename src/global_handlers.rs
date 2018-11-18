use crate::action::{Action, ActionHandled};
use crate::game_event::{GameEvent, GameEventType};
use crate::room::{RoomType, enter_room};
use crate::sound::{AudioEvent, Effect};
use crate::timer::{TimerType};
use crate::App;

// Handle game actions here (Timers).
pub fn handle_action(mut app: &mut App, next_action: Action) {
    match app.try_handle_room_action(&next_action) {
        ActionHandled::Handled => return,
        _ => (),
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
                String::from("System rebooted sucessfully"),
                GameEventType::Success,
            ));
        }
        Action::Enter(room_type) => {
            if room_type == RoomType::Cryocontrol {
                if app.rooms.get(&room_type).unwrap().is_opened() {
                    if room_type != RoomType::Corridor {
                        app.event_queue
                            .schedule_action(Action::Audio(AudioEvent::Effect(
                                Effect::Door,
                            )));
                    }
                    enter_room(&mut app, room_type);
                } else {
                    app.event_queue.schedule_action(Action::Message(
                        String::from("The door is closed and won't open."),
                        GameEventType::Failure,
                    ));
                }
            } else if room_type == RoomType::Corridor {
                if app.rooms.get(&room_type).unwrap().is_opened() {
                    enter_room(&mut app, room_type);
                } else {
                    app.event_queue.schedule_action(Action::Message(
                        String::from(
                            "Peering through the ventilation shafts,\
                              it looks like they connect to a corridor.\
                              You would need a tool to get through.",
                        ),
                        GameEventType::Failure,
                    ));
                }
            } else {
                enter_room(&mut app, room_type);
            }
        }
        Action::Leave(_) => {}
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
            let mut enemy_option = app.state.get_current_enemy_mut(app.state.current_room);
            match enemy_option {
                Some(ref mut enemy) => {
                    enemy.reduce_health(damage);
                    let attack_message = enemy.get_attack_message();
                    if enemy.get_health() <= 0 {
                        app.state.enemies.remove(&app.state.current_room);
                        app.event_queue
                            .schedule_action(Action::Audio(AudioEvent::Effect(
                                Effect::PlayerAttack,
                            )));
                        app.log.push_front(GameEvent {
                            content: String::from("The enemy has been slain.\n"),
                            game_event_type: GameEventType::Failure,
                        });
                        app.event_queue
                            .emplace_timers(TimerType::EnemyAttack, vec![]);
                    }
                    app.log.push_front(GameEvent {
                        content: format!("{}\n", attack_message),
                        game_event_type: GameEventType::Combat,
                    });
                }
                None => {
                    app.event_queue.schedule_action(Action::Message(
                        String::from("There is nothing you can attack."),
                        GameEventType::Failure,
                    ));
                }
            };
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
                        String::from("You dodge the attack. The enemy calmly analyses your movements."),
                        GameEventType::Failure,
                        ));
                    return
                }
                app.event_queue.schedule_action(Action::Message(
                    String::from("You dodge the attack of your own paranoia..."),
                    GameEventType::Failure,
                ));
                return
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
        action => app.log.push_front(GameEvent {
            content: format!("Unhandled action: {}\n", action),
            game_event_type: GameEventType::Debug,
        }),
    }
}

