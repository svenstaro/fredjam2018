extern crate termion;
extern crate tui;

use std::io;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;

mod event;

use self::event::{Event, Events};

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
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    // terminal.hide_cursor()?;

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
            // let v_chunks = Layout::default()
            //     .direction(Direction::Vertical)
            //     .margin(1)
            //     .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            //     .split(size);
            let styled_log = {
                let mut log = vec![];
                for game_event in &app.log {
                    let style = match game_event.game_event_type {
                        GameEventType::Combat => Style::default().fg(Color::Red),
                        GameEventType::Normal => Style::default(),
                        GameEventType::Success => Style::default(),
                        GameEventType::Failure => Style::default(),
                    };
                    log.push(Text::styled(game_event.content.clone(), style));
                }
                log
            };
            Paragraph::new(styled_log.iter())
                .block(Block::default().borders(Borders::ALL).title("Events"))
                .wrap(true)
                .render(&mut f, h_chunks[0]);
            Block::default()
                .title("Map")
                .borders(Borders::ALL)
                .render(&mut f, h_chunks[1]);
        })?;

        match events.next().unwrap() {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
                // if input == Key::Char('s') {
                //     app.log.push(GameEvent { content: "\n");
                // }
            }
            event::Event::Tick => {
                app.log.push(GameEvent { content: "text ".to_string(), game_event_type: GameEventType::Combat });
            }
        }
    }
    Ok(())
}
