use specs::prelude::*;

use crate::{
    components::{Monster, Name, Position, Viewshed},
    maps::Map,
    utils::{Point, a_star::a_star},
};

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, player_point, mut viewsheds, mut positions, monsters, name) = data;

        for (viewshed, monster_position, _, _name) in
            (&mut viewsheds, &mut positions, &monsters, &name).join()
        {
            if !viewshed.visible_tiles.contains(&player_point) {
                continue;
            }

            let path = a_star(Point::from_position(*monster_position), *player_point, &map);
            if path.is_empty() || path.len() < 2 {
                continue;
            }
            monster_position.x = path[1].x;
            monster_position.y = path[1].y;
            viewshed.dirty = true;
        }
    }
}
