use tui::style::{Color, Style};
use tui::widgets::Text;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Item {
    KeyCard,
    Crowbar,
}

#[derive(Debug)]
pub struct Player {
    pub health: i32,
    pub attack_strength: i32,
    pub items: Vec<Item>,
}

impl Player {
    pub fn format_player_info(&self) -> Vec<Text> {
        vec![
            Text::raw("Health: "),
            Text::styled(
                format!("{}", self.health),
                match self.health {
                    0...30 => Style::default().fg(Color::Red),
                    30...70 => Style::default().fg(Color::Yellow),
                    70...100 => Style::default().fg(Color::Green),
                    _ => Style::default(),
                },
            ),
        ]
    }

    pub fn has_item(&self, item: Item) -> bool {
        for owned_item in &self.items {
            if item == *owned_item {
                return true;
            }
        }
        false
    }
}
