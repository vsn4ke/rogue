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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blockers) = data;

        map.populate_blocked_tiles();

        for (position, _) in (&positions, &blockers).join() {
            let index = map.get_index_from_position(*position);
            map.tiles[index].blocked = true;
        }
    }
}
