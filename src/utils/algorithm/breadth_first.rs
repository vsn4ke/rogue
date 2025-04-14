use std::collections::{HashMap, VecDeque};

use crate::{maps::Map, utils::Point};

use super::Pathfinding;

pub struct BreadthFirst;

impl Pathfinding for BreadthFirst {
    fn search(start: Point, end: Point, map: &Map) -> Option<Vec<Point>> {
        let mut frontier = VecDeque::new();
        frontier.push_front(start);

        let mut came_from = HashMap::<Point, Option<Point>>::new();

        while let Some(current) = frontier.pop_front() {
            if current == end {
                break;
            }

            for next in map.get_valid_neighbors(current) {
                came_from.entry(next).or_insert_with(|| {
                    frontier.push_front(next);
                    Some(current)
                });
            }
        }

        let mut current = end;
        let mut path = Vec::new();

        if !came_from.contains_key(&end) {
            return None;
        }

        while current != start {
            path.push(current);

            let came_from = if let Some(c) = came_from.get(&current) {
                c
            } else {
                continue;
            };

            current = if let Some(c) = came_from {
                *c
            } else {
                continue;
            };
        }
        path.reverse();
        Some(path)
    }
}
