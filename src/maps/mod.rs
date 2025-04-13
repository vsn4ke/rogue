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
    pub length: usize,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
}

impl Map {
    pub fn new(width: i32, height: i32, tile: Tile) -> Self {
        let length = (width * height) as usize;
        Self {
            width,
            height,
            length,
            tiles: vec![tile; length],
            rooms: Vec::new(),
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

    pub fn get_index_from_xy(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn get_index_from_position(&self, pos: Position) -> usize {
        self.get_index_from_xy(pos.x, pos.y)
    }

    pub fn get_index_from_point(&self, point: Point) -> usize {
        self.get_index_from_xy(point.x, point.y)
    }

    pub fn get_point_from_index(&self, index: usize) -> Point {
        let x = index as i32 % self.height;
        let y = index as i32 / self.height;
        Point { x, y }
    }

    pub fn insert_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let index = self.get_index_from_xy(x, y);
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

            let index = self.get_index_from_xy(a, b);
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
        let index = self.get_index_from_xy(x, y);
        if self.is_inbound(x, y) && !self.tiles[index].block_path() {
            Some(Point::from_xy(x, y))
        } else {
            None
        }
    }

    pub fn populate_blocked_tiles(&mut self) {
        for i in 0..self.length {
            self.tiles[i].blocked = self.tiles[i].block_path();
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new(TERMINAL_WIDTH, TERMINAL_HEIGHT, Tile::default())
    }
}

pub enum Orientation {
    Vertical,
    Horizontal,
}
