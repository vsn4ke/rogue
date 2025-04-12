pub mod builder;
pub mod tile;

pub use builder::*;
pub use tile::*;

use crate::{
    app::{TERMINAL_HEIGHT, TERMINAL_WIDTH},
    components::Position,
    utils::{Point, Rect, order_value},
};

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
}

impl Map {
    pub fn new(width: i32, height: i32, tile: Tile) -> Self {
        Self {
            width,
            height,
            tiles: vec![tile; (width * height) as usize],
            ..Default::default()
        }
    }
    pub fn is_inbound(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    pub fn is_point_inbound(&self, p: Point) -> bool {
        self.is_inbound(p.x, p.y)
    }

    pub fn is_transparent(&self, index: usize, x: i32, y: i32) -> bool {
        self.is_inbound(x, y) && !self.tiles[index].is_opaque()
    }

    pub fn xy_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn position_to_index(&self, pos: Position) -> usize {
        self.xy_to_index(pos.x, pos.y)
    }

    pub fn point_to_index(&self, point: Point) -> usize {
        self.xy_to_index(point.x, point.y)
    }

    pub fn apply_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let index = self.xy_to_index(x, y);
                self.tiles[index].kind = TileKind::Floor;
            }
        }
    }

    pub fn draw_corridor(&mut self, a1: i32, a2: i32, b: i32, orientation: Orientation) {
        let (a1, a2) = order_value(a1, a2);

        for a in a1..=a2 {
            let (a, b) = match orientation {
                Orientation::Vertical => (b, a),
                Orientation::Horizontal => (a, b),
            };

            if !self.is_inbound(a, b) {
                continue;
            }

            let index = self.xy_to_index(a, b);
            self.tiles[index].kind = TileKind::Floor;
        }
    }

    pub fn clear_visibility(&mut self) {
        for i in 0..(self.width * self.height) as usize {
            self.tiles[i].visible = false;
        }
    }

    pub fn get_exits_from(&self, p: Point) -> Vec<Point> {
        let mut points = Vec::new();

        let delta = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dx, dy) in delta.iter() {
            if let Some(p) = self.get_available_exit(p.x + dx, p.y + dy) {
                points.push(p);
            } else {
                continue;
            }
        }

        points
    }

    fn get_available_exit(&self, x: i32, y: i32) -> Option<Point> {
        let index = self.xy_to_index(x, y);
        if self.is_inbound(x, y) && !self.tiles[index].block_path() {
            Some(Point::from_xy(x, y))
        } else {
            None
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            width: TERMINAL_WIDTH,
            height: TERMINAL_HEIGHT,
            tiles: vec![Tile::default(); (TERMINAL_HEIGHT * TERMINAL_WIDTH) as usize],
            rooms: Vec::new(),
        }
    }
}

pub enum Orientation {
    Vertical,
    Horizontal,
}
