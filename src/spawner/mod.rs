use std::rc::Rc;

use specs::prelude::*;

use crate::{
    colors::*,
    components::{BlockPath, CombatStats, Monster, Name, Player, Position, Renderable, Viewshed},
    maps::Map,
    utils::{Point, Rect, Rng},
};

pub struct Spawn {
    pub name: Rc<str>,
    pub point: Point,
}

impl Spawn {
    pub fn new(name: impl Into<Rc<str>>, point: Point) -> Self {
        Self {
            name: name.into(),
            point,
        }
    }
}

pub fn spawnable_point_in_room(room: &Rect, map: &Map) -> Vec<Point> {
    let mut possibles_spawn_point = Vec::new();

    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            let index = map.get_index_from_xy(x, y);
            if map.tiles[index].is_walkable() {
                possibles_spawn_point.push(Point::from_xy(x, y));
            }
        }
    }

    possibles_spawn_point
}

pub fn build_player_entity(world: &mut World, spawn_point: Point) -> Entity {
    let player = world
        .create_entity()
        .with(Renderable {
            glyph: '@',
            fg: c(BLACK),
            bg: c(RED1),
        })
        .with(Position::from_point(spawn_point))
        .with(Player)
        .with(Name::new("Player"))
        .with(Viewshed::new(8))
        .with(CombatStats::new(50, 2, 6))
        .build();

    player
}

pub fn spawn_monsters(rng: &mut Rng, world: &mut World, map: &Map) {
    for room in map.rooms.iter().skip(1) {
        let spawnable_point = spawnable_point_in_room(room, map);

        let index = rng.random_range(0..spawnable_point.len() as i32) as usize;
        let spawn_point = spawnable_point[index];

        let (glyph, name, stats) = match rng.random_range(0..3) {
            1 => ('o', "Orc", CombatStats::new(20, 2, 4)),
            _ => ('g', "Goblin", CombatStats::new(18, 1, 3)),
        };

        world
            .create_entity()
            .with(Position::from_point(spawn_point))
            .with(Renderable {
                glyph,
                fg: c(RED5),
                ..Default::default()
            })
            .with(Viewshed::new(7))
            .with(Monster)
            .with(Name::new(name))
            .with(BlockPath)
            .with(stats)
            .build();
    }
}
