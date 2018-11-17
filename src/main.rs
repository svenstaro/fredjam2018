use std::io::{self, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

mod event;

use crate::event::{Event, Events};

#[derive(Debug)]
pub enum GameEventType {
    Combat,
    Normal,
    Success,
    Failure,
}

#[derive(Debug)]
pub struct GameEvent {
    pub content: String,
    pub game_event_type: GameEventType,
}

// TODO Extend this to have timers (if needed?)
#[derive(Debug)]
pub enum Action {
    Message(String),
}

pub trait Room {
    fn on_enter(self, state: &mut State) -> bool;
    fn on_leave(self, state: &mut State) -> bool;
    fn on_tick(self, state: &mut State, dt: u32) -> bool;
    fn handle_action(self, state: &mut State, action: Action) -> bool;
}

// Initial room.
#[derive(Default)]
pub struct WakeUp {
}

impl Room for WakeUp {
    fn on_enter(self, state: &mut State) -> bool {
        state.schedule_action(Action::Message(String::from("Welcome to the W.O.R.L.D.")));
        true
    }

    fn on_leave(self, state: &mut State) -> bool {
        true
    }

    fn on_tick(self, state: &mut State, dt: u32) -> bool {
        true
    }

    fn handle_action(self, state: &mut State, action: Action) -> bool {
        true
    }
}

// #[derive(Debug)]
pub struct State {
    pub rooms: Vec<Box<Room>>,
    actions: Vec<Action>,
}

impl State {
    pub fn schedule_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn tick(&mut self, dt: u32) {
        // Tick each room.
    }

    // Return value indicates redraw required.
    pub fn handle_action(&mut self, action: &Action) -> bool {
        // Try handling the action in a room, if that succeeds, then break, else try handling
        // globally.
        true
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            rooms: vec![
                Box::new(WakeUp{})
            ],
            actions: vec![],
        }
    }
}

#[derive(Default)]
pub struct App {
    pub size: Rect,
    pub log: Vec<GameEvent>,
    pub input: String,
    pub state: State,
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let mut app: App = Default::default();

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
            Block::default()
                .title("Map")
                .borders(Borders::ALL)
                .render(&mut f, h_chunks[1]);
        })?;

        write!(
            terminal.backend_mut(),
            "{}",
            Goto(4 + app.input.width() as u16, 3)
        )?;

        // Handle system events.
        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Esc => {
                    break;
                }
                Key::Char('\n') => {
                    app.log.push(GameEvent { content: app.input.drain(..).collect(), game_event_type: GameEventType::Normal });
                }
                Key::Char(c) => {
                    app.input.push(c);
                }
                Key::Backspace => {
                    app.input.pop();
                }
                _ => {}
            }
            event::Event::Tick => {
                app.log.push(GameEvent { content: "text ".to_string(), game_event_type: GameEventType::Combat });
                app.state.tick(0); // TODO dt how?
            }
        }

        // Handle game actions here (Timers).
    }
    Ok(())
}
