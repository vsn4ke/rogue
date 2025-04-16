use crate::{maps::Map, utils::Rng};

use super::{MapBuilder, initial::RandomRooms, meta::Corridors};

pub fn rooms_and_corridors(rng: &mut Rng) -> Map {
    MapBuilder::new(RandomRooms::new(4, 5, 8))
        .with(Corridors::new())
        .build(rng)
}
