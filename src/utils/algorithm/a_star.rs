use crate::{maps::Map, utils::Point};
use std::collections::HashMap;

use super::Pathfinding;

#[derive(Clone, Copy, Debug)]
struct Node {
    pub p: Point,
    pub f: i32,
    pub parent: Option<Point>,
}

impl Node {
    pub fn new(p: Point, start: Point, goal: Point, parent: Option<Point>) -> Self {
        let g = p.distance_squared_to(start);
        let h = p.distance_squared_to(goal);
        let f = g + h;

        Self { p, f, parent }
    }
}

pub struct AStar;

impl Pathfinding for AStar {
    fn search(start: Point, goal: Point, map: &Map) -> Option<Vec<Point>> {
        let mut opened_node: HashMap<Point, Node> = HashMap::new();
        let mut closed_node: HashMap<Point, Node> = HashMap::new();
        let mut path = Vec::new();
        opened_node.insert(start, Node::new(start, start, goal, None));

        for _ in 0..100 {
            let mut lowest_f = i32::MAX;
            let mut current_node = None;

            for (_, node) in opened_node.iter() {
                if node.f < lowest_f {
                    lowest_f = node.f;
                    current_node = Some(*node);
                }
            }

            let current_node = if let Some(c) = current_node {
                c
            } else {
                break;
            };

            let mut current_position = current_node.p;

            opened_node.remove(&current_position);

            closed_node.insert(
                current_position,
                Node::new(current_position, start, goal, current_node.parent),
            );

            if current_position == goal {
                let mut nodelist = Vec::new();
                loop {
                    nodelist.push(current_position);
                    let node = if let Some(n) = closed_node.get(&current_position) {
                        n
                    } else {
                        break;
                    };

                    current_position = if let Some(n) = node.parent {
                        n
                    } else {
                        break;
                    };
                }

                path = nodelist;
                break;
            }

            for point in map.get_valid_neighbors(current_position).iter() {
                if closed_node.contains_key(point) {
                    continue;
                }
                opened_node.insert(
                    *point,
                    Node::new(*point, start, goal, Some(current_position)),
                );
            }
        }

        path.pop();
        path.reverse();

        Some(path)
    }
}
