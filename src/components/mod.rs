use ratatui::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
pub use units::*;

use crate::{
    colors::{BLACK, WHITE, c},
    utils::Point,
};

pub mod units;

#[derive(Component, Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn from_point(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
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
            fg: c(WHITE),
            bg: c(BLACK),
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

#[derive(Component)]
pub struct BlockPath;
