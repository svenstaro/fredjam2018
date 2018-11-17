use itertools::Itertools;
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
}

#[derive(Debug)]
pub struct GameEvent {
    pub content: String,
    pub game_event_type: GameEventType,
}

#[derive(Debug, Default)]
pub struct App {
    pub size: Rect,
    pub log: Vec<GameEvent>,
    pub input: String,
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
            Goto(4 + app.input.width() as u16, 3)
        )?;

        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Esc => {
                    break;
                }
                Key::Char('\n') => {
                    app.log.push(GameEvent {
                        content: app.input.drain(..).collect(),
                        game_event_type: GameEventType::Normal,
                    });
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
                app.log.push(GameEvent {
                    content: "text ".to_string(),
                    game_event_type: GameEventType::Combat,
                });
            }
        }
    }
    Ok(())
}
