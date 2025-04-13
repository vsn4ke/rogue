use crate::app::{TERMINAL_HEIGHT, TERMINAL_WIDTH};
use crate::maps::{Map, Tile, TileKind};
use crate::utils::Rng;

pub fn simple_map() -> Map {
    let mut map = Map::new(
        TERMINAL_WIDTH,
        TERMINAL_HEIGHT,
        Tile {
            kind: TileKind::Floor,
            ..Default::default()
        },
    );

    for x in 0..map.width {
        let index = map.get_index_from_xy(x, 0);
        map.tiles[index].kind = TileKind::Wall;
        let index = map.get_index_from_xy(x, map.height - 1);
        map.tiles[index].kind = TileKind::Wall;
    }

    for y in 0..map.height {
        let index = map.get_index_from_xy(0, y);
        map.tiles[index].kind = TileKind::Wall;
        let index = map.get_index_from_xy(map.width - 1, y);
        map.tiles[index].kind = TileKind::Wall;
    }

    let mut rng = Rng::random_seed();

    for _ in 0..400 {
        let x = rng.random_range(1..map.width);
        let y = rng.random_range(1..map.height);
        let index = map.get_index_from_xy(x, y);
        if x != 30 && y != 40 {
            map.tiles[index].kind = TileKind::Wall;
        }
    }

    map
}
