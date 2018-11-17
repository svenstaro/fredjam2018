use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Line, Shape};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

mod event;

use crate::event::{Event, Events};

pub struct BoxShape {
    pub rect: Rect,
    pub color: Color,
}

impl<'a> Shape<'a> for BoxShape {
    fn color(&self) -> Color {
        self.color
    }

    fn points(&'a self) -> Box<Iterator<Item = (f64, f64)> + 'a> {
        let left_line = Line {
            x1: f64::from(self.rect.x),
            y1: f64::from(self.rect.y),
            x2: f64::from(self.rect.x),
            y2: f64::from(self.rect.y + self.rect.height),
            color: self.color,
        };
        let top_line = Line {
            x1: f64::from(self.rect.x),
            y1: f64::from(self.rect.y + self.rect.height),
            x2: f64::from(self.rect.x + self.rect.width),
            y2: f64::from(self.rect.y + self.rect.height),
            color: self.color,
        };
        let right_line = Line {
            x1: f64::from(self.rect.x + self.rect.width),
            y1: f64::from(self.rect.y),
            x2: f64::from(self.rect.x + self.rect.width),
            y2: f64::from(self.rect.y + self.rect.height),
            color: self.color,
        };
        let bottom_line = Line {
            x1: f64::from(self.rect.x),
            y1: f64::from(self.rect.y),
            x2: f64::from(self.rect.x + self.rect.width),
            y2: f64::from(self.rect.y),
            color: self.color,
        };
        Box::new(
            left_line.into_iter().merge(
                top_line
                    .into_iter()
                    .merge(right_line.into_iter().merge(bottom_line.into_iter())),
            ),
        )
    }
}

#[derive(Debug)]
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
pub enum Action {
    // String is room name.
    Enter(&'static str),
    Tick(u32),
    // String is room name.
    Leave(&'static str),
    Message(String, GameEventType),
    Command(String),
}

pub trait Room {
    fn handle_action(&mut self, state: &mut State, action: &Action) -> bool;
}

// Initial room.
pub struct WakeUpRoom {
    pub lever: bool,
}

impl Room for WakeUpRoom {
    fn handle_action(&mut self, state: &mut State, action: &Action) -> bool {
        match action {
            Action::Enter(room) => {
                if room == &"WakeUp" {
                    state.schedule_action(Action::Message(
                        String::from("Welcome to W.O.R.L.D."),
                        GameEventType::Success,
                    ));
                    true
                } else {
                    false
                }
            }
            Action::Command(command) => {
                // TODO Maybe there's a better approach to finding the current room...
                if state.current_room == "WakeUp" {
                    // TODO replace command by proper enum.
                    if command == &"use lever" {
                        if !self.lever {
                            state.schedule_action(Action::Message(
                                String::from(
                                    "You pull the lever, it flips down with a screeching noize.",
                                ),
                                GameEventType::Success,
                            ));
                            self.lever = true;
                        } else {
                            state.schedule_action(Action::Message(
                                String::from("There is no point in pulling this back up."),
                                GameEventType::Failure,
                            ));
                        }
                        true
                    } else if command == &"look around" {
                        state.schedule_action(Action::Message(
                            String::from("The room is empty, there is just some lever and a solid door with no handle."),
                            GameEventType::Normal,
                        ));
                        true
                    } else if command == &"use door" {
                        if self.lever {
                            state.schedule_action(Action::Message(
                                String::from("You open the door and pass through."),
                                GameEventType::Success,
                            ));
                            state.schedule_action(Action::Leave("WakeUp"));
                            state.schedule_action(Action::Enter("Locked"));
                            true
                        } else {
                            state.schedule_action(Action::Message(
                                String::from("No matter how hard you push the door, it does not even move an inch."),
                                GameEventType::Failure,
                            ));
                            true
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

// Second room room, locked per default, lever needs to be pulled.
pub struct LockedRoom;

impl Room for LockedRoom {
    fn handle_action(&mut self, state: &mut State, action: &Action) -> bool {
        match action {
            Action::Enter(room) => {
                if room == &"Locked" {
                    state.schedule_action(Action::Message(
                        String::from(
                            "Success, you have entered the second and final room! You win!",
                        ),
                        GameEventType::Success,
                    ));
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

pub struct State {
    pub rooms: HashMap<&'static str, Box<Room>>,
    pub current_room: &'static str,
    actions: VecDeque<Action>,
}

impl State {
    pub fn schedule_action(&mut self, action: Action) {
        self.actions.push_back(action);
    }

    // Return value indicates redraw required.
    pub fn try_handle_room_action(&mut self, action: &Action) -> bool {
        // Try handling the action in a room, if that succeeds, then return true.
        let mut keys = vec![];
        for key in self.rooms.keys() {
            keys.push(*key);
        }

        for name in keys {
            let mut room = self.rooms.remove(name).unwrap();
            let handled = room.handle_action(self, action);
            self.rooms.insert(name, room);
            if handled {
                return true;
            }
        }
        false
    }
}

impl State {
    fn new() -> Self {
        let mut rooms: HashMap<&'static str, Box<Room>> = HashMap::new();
        rooms.insert("WakeUp", Box::new(WakeUpRoom { lever: false }));
        rooms.insert("Locked", Box::new(LockedRoom {}));
        State {
            rooms,
            current_room: "WakeUp",
            actions: VecDeque::new(),
        }
    }
}

pub struct App {
    pub size: Rect,
    pub log: Vec<GameEvent>,
    pub input: String,
    pub state: State,
}

impl App {
    fn new(state: State) -> Self {
        App {
            size: Default::default(),
            log: vec![],
            input: "".into(),
            state: state,
        }
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let state = State::new();
    let mut app = App::new(state);

    let mut wake_up = app.state.rooms.remove("WakeUp").unwrap();
    wake_up.handle_action(&mut app.state, &Action::Enter("WakeUp"));
    app.state.rooms.insert("WakeUp", wake_up);

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
                    app.state.schedule_action(command);
                }
                Key::Char(c) => {
                    app.input.push(c);
                }
                Key::Backspace => {
                    app.input.pop();
                }
                _ => {}
            },
            event::Event::Tick => {
                // TODO dt how?
                app.state.schedule_action(Action::Tick(0));
            }
        }

        // Handle game actions here (Timers).
        while !app.state.actions.is_empty() {
            let next_action = app.state.actions.pop_front().unwrap();
            let handled = app.state.try_handle_room_action(&next_action);
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
                Action::Tick(dt) => {}
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
