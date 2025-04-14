use crate::maps::Map;

use super::{MapBuilder, initial::Empty, meta::RoomsAndCorridors};

pub fn rooms_and_corridors() -> Map {
    MapBuilder::new(Empty::new())
        .with(RoomsAndCorridors::new(4, 4, 10))
        .build()
}
