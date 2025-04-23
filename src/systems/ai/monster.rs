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

        for (viewshed, monster_position, _, name) in
            (&mut viewsheds, &mut positions, &monsters, &name).join()
        {
            if !viewshed.visible_tiles.contains(&player_point) {
                continue;
            }

            let monster_point = Point::from_position(*monster_position);
            let distance = monster_point.distance_squared_to(*player_point);
            if distance < 10 {
                logger.add_entries(format!("{} shouts insults from {} meters", name, distance));
                continue;
            }

            let Some(path) = AStar::search(monster_point, *player_point, &map) else {
                continue;
            };

            monster_position.x = path[0].x;
            monster_position.y = path[0].y;
            let index = map.get_index_from_point(path[0]);
            map.tiles[index].blocked = true;

            viewshed.dirty = true;
        }
    }
}
