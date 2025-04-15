use ratatui::style::Color;
use specs::Entity;

use crate::components::Renderable;

#[derive(Clone, PartialEq, Copy)]
pub enum TileKind {
    Wall,
    Floor,
}

#[derive(Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub revealed: bool,
    pub visible: bool,
    pub blocked: bool,
    pub content: Vec<Entity>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            kind: TileKind::Wall,
            revealed: false,
            visible: false,
            blocked: false,
            content: Vec::new(),
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
        matches!(self.kind, TileKind::Wall)
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked_path() || self.blocked
    }

    pub fn blocked_path(&self) -> bool {
        matches!(self.kind, TileKind::Wall)
    }
}
