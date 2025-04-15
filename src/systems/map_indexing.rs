use specs::prelude::*;

use crate::{
    components::{BlockPath, Position},
    maps::Map,
};

pub struct MapIndexing;

impl<'a> System<'a> for MapIndexing {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlockPath>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blockers, entities) = data;

        map.populate_blocked_tiles();
        map.clear_tiles_content();

        for (position, entity) in (&positions, &entities).join() {
            let index = map.get_index_from_position(*position);

            map.tiles[index].blocked = blockers.get(entity).is_some();
            map.tiles[index].content.push(entity);
        }
    }
}
