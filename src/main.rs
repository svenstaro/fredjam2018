extern crate num;

use num::clamp;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::canvas::Canvas;
use tui::widgets::{Block, Borders, Gauge, Paragraph, Text, Widget};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

#[macro_use]
extern crate strum_macros;

mod action;
mod commands;
mod entities;
mod event;
mod event_queue;
mod game_event;
mod global_handlers;
mod room;
mod rooms;
mod sound;
mod state;
mod timer;
mod utils;

use crate::action::{Action, ActionHandled};
use crate::commands::try_handle_command;
use crate::entities::enemy::initialize_enemies;
use crate::event::{Event, Events};
use crate::event_queue::EventQueue;
use crate::game_event::{GameEvent, GameEventType};
use crate::global_handlers::handle_action;
use crate::room::{Room, RoomType};
use crate::rooms::{CorridorRoom, CryobayRoom, Cryocontrol, SlushLobbyRoom};
use crate::sound::{AudioEvent, Effect};

use crate::state::State;
use crate::utils::{duration_to_msec_u64, BoxShape};

#[derive(Debug)]
pub struct App {
    // The size of the console window.
    pub size: Rect,
    // The system event, like rendering stuff in the console.
    pub log: VecDeque<GameEvent>,
    // The input in the command box.
    pub input: String,
    // The global game state.
    pub state: State,
    // The list of rooms.
    pub rooms: HashMap<RoomType, Box<Room>>,
    // The action event queue.
    pub event_queue: EventQueue,
}

impl App {
    fn new(state: State) -> Self {
        App {
            size: Default::default(),
            log: Default::default(),
            input: "".into(),
            state: state,
            rooms: Default::default(),
            event_queue: Default::default(),
        }
    }

    // Return value indicates redraw required.
    pub fn try_handle_room_action(&mut self, action: &Action) -> ActionHandled {
        // Try handling the action in a room, if that succeeds, then return true.
        for (_, ref mut room) in &mut self.rooms {
            match room.handle_action(&mut self.state, &mut self.event_queue, action) {
                ActionHandled::Handled => return ActionHandled::Handled,
                _ => (),
            }
        }

        ActionHandled::NotHandled
    }

    pub fn try_handle_command(&mut self, tokens: String) {
        let actions = try_handle_command(tokens, &self.state);
        self.event_queue.schedule_actions(actions);
    }
}

fn main() -> Result<(), io::Error> {
    let (snd_send, snd_recv) = channel();

    snd_send.send(AudioEvent::Effect(Effect::Typing));
    thread::spawn(move || {
        sound::start(snd_recv);
    });

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let mut state = State::new();
    let mut app = App::new(state);

    app.rooms
        .insert(RoomType::Cryobay, Box::new(CryobayRoom::new()));
    app.rooms
        .insert(RoomType::Cryocontrol, Box::new(Cryocontrol::new()));
    app.rooms
        .insert(RoomType::SlushLobby, Box::new(SlushLobbyRoom::new()));
    app.rooms
        .insert(RoomType::Corridor, Box::new(CorridorRoom::new()));

    app.event_queue
        .schedule_action(Action::Enter(RoomType::Cryobay));

    let mut now = Instant::now();
    initialize_enemies(&mut app.state);

    loop {
        let size = terminal.size()?;
        if size != app.size {
            terminal.resize(size)?;
            app.size = size;
        }

        // Draw.
        terminal.draw(|mut f| {
            let h_chunks = Layout::default()
                // Split along the horizontal axis.
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);
            let v_chunks_left = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .split(h_chunks[0]);
            let input_status_line = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(v_chunks_left[0]);
            let v_chunks_right = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(h_chunks[1]);
            let v_chunks_right_up = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Max(3),
                        Constraint::Max(3),
                        Constraint::Max(3),
                        Constraint::Max(3),
                        Constraint::Max(3),
                        Constraint::Max(0),
                    ]
                    .as_ref(),
                )
                .split(v_chunks_right[0]);

            let styled_log = {
                let mut log = vec![];
                for game_event in &app.log {
                    let style = match game_event.game_event_type {
                        GameEventType::Combat => Style::default().fg(Color::Red),
                        GameEventType::Normal => Style::default(),
                        GameEventType::Success => Style::default().fg(Color::Green),
                        GameEventType::Failure => Style::default().fg(Color::Red),
                        GameEventType::Debug => Style::default().fg(Color::Blue),
                    };
                    log.push(Text::styled(game_event.content.clone(), style));
                }
                log
            };

            if cfg!(debug_assertions) {
                Paragraph::new([Text::raw("DEV MODE: no movement restrictions + other cheats enabled")].iter())
                    .style(Style::default().fg(Color::Red))
                    .render(&mut f, *v_chunks_right_up.last().unwrap())
            }

            Paragraph::new(styled_log.iter())
                .block(Block::default().borders(Borders::ALL).title("Events"))
                .wrap(true)
                .render(&mut f, v_chunks_left[1]);
            Paragraph::new([Text::raw(&app.input)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .render(&mut f, input_status_line[0]);
            Paragraph::new(app.state.player.format_player_info().iter())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL).title("Character"))
                .render(&mut f, input_status_line[1]);

            Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Map"))
                .paint(|ctx| {
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 20,
                            y: 70,
                            width: 20,
                            height: 20,
                        },
                        color: match app.state.current_room {
                            RoomType::Cryobay => Color::Red,
                            _ => Color::White,
                        },
                    });
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 30,
                            y: 60,
                            width: 5,
                            height: 10,
                        },
                        color: Color::White,
                    });
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 20,
                            y: 40,
                            width: 20,
                            height: 20,
                        },
                        color: match app.state.current_room {
                            RoomType::SlushLobby => Color::Red,
                            _ => Color::White,
                        },
                    });
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 40,
                            y: 45,
                            width: 10,
                            height: 5,
                        },
                        color: Color::White,
                    });
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 50,
                            y: 40,
                            width: 20,
                            height: 20,
                        },
                        color: match app.state.current_room {
                            RoomType::Cryocontrol => Color::Red,
                            _ => Color::White,
                        },
                    });
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 24,
                            y: 17,
                            width: 2,
                            height: 22,
                        },
                        color: Color::White,
                    });
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 20,
                            y: 5,
                            width: 35,
                            height: 12,
                        },
                        color: match app.state.current_room {
                            RoomType::Corridor => Color::Blue,
                            _ => Color::White,
                        },
                    });
                })
                .x_bounds([0.0, 100.0])
                .y_bounds([0.0, 100.0])
                .render(&mut f, v_chunks_right[1]);

            let visible_timers = app
                .event_queue
                .timers
                .iter()
                .filter(|timer| timer.is_visual);

            for (index, timer) in visible_timers.enumerate() {
                // Only render the first 5 timers.
                if index > 4 {
                    break;
                }

                let int_progress = clamp(
                    (timer.duration as i64 - timer.elapsed as i64) * 100i64 / timer.duration as i64,
                    0,
                    100,
                ) as u16;
                Gauge::default()
                    .block(Block::default().title(&timer.label).borders(Borders::ALL))
                    .style(Style::default().fg(Color::Magenta).bg(Color::Green))
                    .percent(int_progress)
                    .label(&format!("{}", int_progress))
                    .render(&mut f, v_chunks_right_up[index]);
            }
        })?;

        write!(
            terminal.backend_mut(),
            "{}",
            Goto(3 + app.input.width() as u16, 3)
        )?;

        terminal.backend_mut().flush()?;

        // Handle system events.
        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Esc => {
                    break;
                }
                Key::Char('\n') => {
                    if !app.input.is_empty() {
                        let mut content: String = app.input.drain(..).collect();
                        let command = Action::Command(content.clone());
                        content = format!("\n>>> {}", content);
                        content.push_str("\n\n");
                        app.log.push_front(GameEvent {
                            content: content,
                            game_event_type: GameEventType::Normal,
                        });
                        app.event_queue.schedule_action(command);
                    }
                }
                Key::Char(c) => {
                    snd_send.send(AudioEvent::Effect(Effect::Typing));
                    if app.input.is_empty() {
                        app.event_queue.schedule_action(Action::PlayerFinishedReading)
                    }
                    app.input.push(c);
                }
                Key::Backspace => {
                    snd_send.send(AudioEvent::Effect(Effect::Backspace));
                    app.input.pop();
                }
                _ => {}
            },
            event::Event::Tick => {
                let elapsed = duration_to_msec_u64(&now.elapsed());
                app.event_queue.schedule_action(Action::Tick(elapsed));
            }
        }

        while let Some(next_action) = app.event_queue.get_next_action() {
            match next_action {
                Action::Audio(action) => {
                    snd_send.send(action).unwrap();
                    continue;
                },
                _ => (),
            }

            handle_action(&mut app, next_action);
        }

        now = Instant::now();
    }
    Ok(())
}
