use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::canvas::Canvas;
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;
use std::thread;
use std::sync::mpsc::channel;
use self::sound::{AudioEvent, Effect};

mod event;
mod sound;
mod rooms;
mod utils;

use crate::event::{Event, Events};
use crate::rooms::{LockedRoom, Room, RoomType, WakeUpRoom};
use crate::utils::BoxShape;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GameEventType {
    Combat,
    Normal,
    Success,
    Failure,
    Debug,
}

#[derive(Debug)]
pub struct GameEvent {
    pub content: String,
    pub game_event_type: GameEventType,
}

// TODO Extend this to have timers (if needed?)
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Action {
    // String is room name.
    Enter(RoomType),
    Tick(u32),
    // String is room name.
    Leave(RoomType),
    Message(String, GameEventType),
    Command(String),
}

#[derive(Debug)]
pub struct State {
    pub current_room: RoomType,
}

impl State {
    fn new() -> Self {
        State {
            current_room: RoomType::WakeUp,
        }
    }
}

#[derive(Debug, Default)]
pub struct EventQueue {
    pub actions: VecDeque<Action>,
}

impl EventQueue {
    pub fn schedule_action(&mut self, action: Action) {
        self.actions.push_back(action);
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn get_next_action(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }
}

#[derive(Debug)]
pub struct App {
    pub size: Rect,
    pub log: Vec<GameEvent>,
    pub input: String,
    pub state: State,
    pub rooms: HashMap<RoomType, Box<Room>>,
    pub event_queue: EventQueue,
}

impl App {
    fn new(state: State) -> Self {
        App {
            size: Default::default(),
            log: vec![],
            input: "".into(),
            state: state,
            rooms: HashMap::new(),
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
        .insert(RoomType::WakeUp, Box::new(WakeUpRoom { lever: false }));
    app.rooms.insert(RoomType::Locked, Box::new(LockedRoom {}));

    app.event_queue
        .schedule_action(Action::Enter(RoomType::WakeUp));

    loop {
        let size = terminal.size()?;
        if size != app.size {
            terminal.resize(size)?;
            app.size = size;
        }

        terminal.draw(|mut f| {
            let h_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);
            let v_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .split(h_chunks[0]);
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
                .render(&mut f, v_chunks[1]);
            Paragraph::new([Text::raw(&app.input)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Events"))
                .render(&mut f, v_chunks[0]);
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
                .render(&mut f, h_chunks[1]);
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
                    content.push('\n');
                    app.log.push(GameEvent {
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
                // TODO dt how?
                app.event_queue.schedule_action(Action::Tick(0));
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
                    app.log.push(GameEvent {
                        content: message,
                        game_event_type,
                    })
                }
                Action::Tick(_dt) => {}
                _ => app.log.push(GameEvent {
                    content: String::from("Unhandled action!\n"),
                    game_event_type: GameEventType::Debug,
                }),
            }
        }

        // Uncomment to print stuff in each iteration.
        // app.log.push(GameEvent { content: String::from("ASDASD"), game_event_type: GameEventType::Combat })
    }
    Ok(())
}
