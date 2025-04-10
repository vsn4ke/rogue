use std::mem::swap;

use crate::components::Position;

#[derive(Clone, Copy)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            x2: x + w,
            y1: y,
            y2: y + h,
        }
    }

    pub fn intersect(self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn center_position(&self) -> Position {
        let (x, y) = self.center();
        Position { x, y }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn from_position(pos: Position) -> Self {
        Self { x: pos.x, y: pos.y }
    }

    pub fn from_xy(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_xy(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

/// Bresenham's Line Algorithm
pub fn get_line(a: Point, b: Point) -> Vec<Point> {
    let mut points = Vec::<Point>::new();
    let (mut x1, mut y1) = (a.x, a.y);
    let (mut x2, mut y2) = (b.x, b.y);

    let is_steep = (y2 - y1).abs() > (x2 - x1).abs();
    if is_steep {
        swap(&mut x1, &mut y1);
        swap(&mut x2, &mut y2);
    }

    let mut reversed = false;
    if x1 > x2 {
        swap(&mut x1, &mut x2);
        swap(&mut y1, &mut y2);
        reversed = true;
    }

    let dx = x2 - x1;
    let dy = (y2 - y1).abs();

    let mut err = dx / 2;
    let mut y = y1;
    let y_step = if y1 < y2 { 1 } else { -1 };

    for x in x1..x2 + 1 {
        points.push(if is_steep {
            Point { x: y, y: x }
        } else {
            Point { x, y }
        });

        err -= dy;
        if err < 0 {
            y += y_step;
            err += dx;
        }
    }

    if reversed {
        for i in 0..points.len() / 2 {
            let end = points.len() - 1;
            points.swap(i, end - i);
        }
    }

    points
}
