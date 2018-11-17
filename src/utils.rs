use itertools::Itertools;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas::{Line, Shape};

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
