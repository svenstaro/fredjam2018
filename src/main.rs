use self::sound::{AudioEvent, Effect, Track};
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

mod action;
mod enemy;
mod event;
mod event_queue;
mod game_event;
mod player;
mod rooms;
mod sound;
mod state;
mod timer;
mod utils;

use crate::action::Action;
use crate::event::{Event, Events};
use crate::event_queue::EventQueue;
use crate::game_event::{GameEvent, GameEventType};
use crate::rooms::{room_intro_text, CryobayRoom, Room, RoomType, SlushLobbyRoom};
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
    pub fn try_handle_room_action(&mut self, action: &Action) -> bool {
        // Try handling the action in a room, if that succeeds, then return true.
        for (_, ref mut room) in &mut self.rooms {
            let handled = room.handle_action(&mut self.state, &mut self.event_queue, action);
            if handled {
                return true;
            }
        }
        false
    }
}

fn main() -> Result<(), io::Error> {
    let (snd_send, snd_recv) = channel();

    snd_send.send(AudioEvent::Effect(Effect::BeepLong));
    thread::spawn(move || {
        sound::start(snd_recv);
    });

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let state = State::new();
    let mut app = App::new(state);

    app.rooms
        .insert(RoomType::Cryobay, Box::new(CryobayRoom::new()));
    app.rooms
        .insert(RoomType::SlushLobby, Box::new(SlushLobbyRoom::new()));

    app.event_queue
        .schedule_action(Action::Enter(RoomType::Cryobay));

    let mut now = Instant::now();

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
            let v_chunks_right = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(h_chunks[1]);
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
            Paragraph::new(styled_log.iter())
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .wrap(true)
                .render(&mut f, v_chunks_left[1]);
            Paragraph::new([Text::raw(&app.input)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Events"))
                .render(&mut f, v_chunks_left[0]);
            Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Map"))
                .paint(|ctx| {
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 50,
                            y: 50,
                            width: 20,
                            height: 20,
                        },
                        color: Color::White,
                    });
                    ctx.draw(&BoxShape {
                        rect: Rect {
                            x: 20,
                            y: 20,
                            width: 20,
                            height: 20,
                        },
                        color: Color::White,
                    });
                })
                .x_bounds([0.0, 100.0])
                .y_bounds([0.0, 100.0])
                .render(&mut f, v_chunks_right[1]);
            for timer in &app.event_queue.timers {
                Gauge::default()
                    .block(Block::default().title("TODO label").borders(Borders::ALL))
                    .style(Style::default().fg(Color::Magenta).bg(Color::Green))
                    .percent(50)
                    .label(&format!("Gauge label {}/100", 50))
                    .render(&mut f, v_chunks_right[0]);
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
                    let mut content: String = app.input.drain(..).collect();
                    let command = Action::Command(content.clone());
                    snd_send.send(AudioEvent::Track(Track::Intro));
                    content.push('\n');
                    app.log.push_front(GameEvent {
                        content,
                        game_event_type: GameEventType::Normal,
                    });
                    app.event_queue.schedule_action(command);
                }
                Key::Char(c) => {
                    snd_send.send(AudioEvent::Effect(Effect::BeepLong));
                    app.input.push(c);
                }
                Key::Backspace => {
                    app.input.pop();
                }
                _ => {}
            },
            event::Event::Tick => {
                let elapsed = duration_to_msec_u64(&now.elapsed());
                app.event_queue.schedule_action(Action::Tick(elapsed));
            }
        }

        // Handle game actions here (Timers).
        while !app.event_queue.is_empty() {
            let next_action = app.event_queue.get_next_action().unwrap();
            let handled = app.try_handle_room_action(&next_action);
            if handled {
                break;
            }

            // Handle system and global actions here.
            match next_action {
                Action::Message(mut message, game_event_type) => {
                    message.push('\n');
                    app.log.push_front(GameEvent {
                        content: message,
                        game_event_type,
                    })
                }
                Action::Enter(room) => {
                    app.event_queue.schedule_action(Action::Message(
                        String::from(room_intro_text(room)),
                        GameEventType::Normal,
                    ));
                }
                Action::Tick(dt) => {
                    app.event_queue.tick(dt);
                }
                _ => app.log.push_front(GameEvent {
                    content: String::from("Unhandled action!\n"),
                    game_event_type: GameEventType::Debug,
                }),
            }
        }

        now = Instant::now();
    }
    Ok(())
}
