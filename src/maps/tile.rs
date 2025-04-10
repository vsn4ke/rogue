use ratatui::style::Color;

use crate::components::Renderable;

#[derive(Clone, PartialEq, Copy)]
pub enum TileKind {
    Wall,
    Floor,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub kind: TileKind,
    pub revealed: bool,
    pub visible: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            kind: TileKind::Wall,
            revealed: false,
            visible: false,
        }
    }
}

impl Tile {
    pub fn renderable(&self) -> Renderable {
        let mut renderable = match self.kind {
            TileKind::Wall => Renderable {
                glyph: '#',
                fg: Color::Green,
                ..Default::default()
            },
            TileKind::Floor => Renderable {
                glyph: '.',
                fg: Color::Green,
                ..Default::default()
            },
        };

        if !self.visible {
            renderable.fg = Color::DarkGray;
        }

        renderable
    }

    pub fn is_opaque(&self) -> bool {
        match self.kind {
            TileKind::Wall => true,
            _ => false,
        }
    }
}
