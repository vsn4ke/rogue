use specs::prelude::*;

use crate::{
    app::logger::Logger,
    components::{Monster, Name, Position, Viewshed},
    maps::Map,
    utils::{Pathfinding, Point, a_star::AStar},
};

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, Logger>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_point, mut viewsheds, mut positions, monsters, name, mut logger) =
            data;

        for (viewshed, monster_position, _, _name) in
            (&mut viewsheds, &mut positions, &monsters, &name).join()
        {
            if !viewshed.visible_tiles.contains(&player_point) {
                continue;
            }

            map.populate_blocked_tiles();

            if let Some(path) =
                AStar::search(Point::from_position(*monster_position), *player_point, &map)
            {
                logger.add_entries(format!("{:?}", path));
                monster_position.x = path[0].x;
                monster_position.y = path[0].y;
            } else {
                continue;
            }

            viewshed.dirty = true;
        }
    }
}
