use crate::maps::Map;

use super::Point;

pub mod a_star;
pub mod breadth_first;

pub trait Pathfinding {
    fn search(start: Point, end: Point, map: &Map) -> Option<Vec<Point>>;
}
