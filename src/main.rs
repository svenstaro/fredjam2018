extern crate termion;
extern crate tui;

use std::io;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget, Paragraph};
use tui::Terminal;

mod event;

use self::event::{Event, Events};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;

    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(size);
            Block::default()
                .title("World")
                .borders(Borders::ALL)
                .render(&mut f, chunks[0]);
            Block::default()
                .title("Map")
                .borders(Borders::ALL)
                .render(&mut f, chunks[1]);
        })?;

        match events.next().unwrap() {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
            }
            event::Event::Tick => {
                // println!("hallo hans");
            }
        }
    }
    Ok(())
}
