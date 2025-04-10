use ratatui::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
pub use units::*;

pub mod units;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Renderable {
    fn default() -> Renderable {
        Renderable {
            glyph: ' ',
            fg: Color::White,
            bg: Color::Black,
        }
    }
}

impl Renderable {
    pub fn draw<'a>(self) -> Span<'a> {
        Span {
            style: Style {
                fg: Some(self.fg),
                bg: Some(self.bg),
                ..Default::default()
            },
            content: self.glyph.to_string().into(),
        }
    }
}
