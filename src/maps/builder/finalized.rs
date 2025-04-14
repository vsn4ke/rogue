use crate::maps::Map;

use super::{MapBuilder, initial::RandomRooms, meta::Corridors};

pub fn rooms_and_corridors() -> Map {
    MapBuilder::new(RandomRooms::new(4, 5, 8))
        .with(Corridors::new())
        .build()
}
