use crate::{
    maps::{Map, MetaBuilder, TileKind},
    utils::{Rng, order_value},
};

pub struct Corridors;

impl Corridors {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl MetaBuilder for Corridors {
    fn draw(&self, map: &mut Map) {
        if map.rooms.is_none() {
            return;
        }

        let rooms = map.rooms.as_mut().unwrap().clone();

        let last_room = rooms.len() - 1;
        for i in 0..=last_room {
            if i + 1 > last_room {
                break;
            }
            let (new_x, new_y) = rooms[i].center();
            let (prev_x, prev_y) = rooms[i + 1].center();

            let mut rng = Rng::random_seed();
            let (middle_x, middle_y) = if rng.random_range(0..2) == 1 {
                (new_x, prev_y)
            } else {
                (prev_x, new_y)
            };

            draw_corridor(map, prev_x, new_x, middle_y, Orientation::Horizontal);
            draw_corridor(map, prev_y, new_y, middle_x, Orientation::Vertical);
        }
    }
}

fn draw_corridor(map: &mut Map, a1: i32, a2: i32, b: i32, orientation: Orientation) {
    let (a1, a2) = order_value(a1, a2);

    for a in a1..=a2 {
        let (a, b) = match orientation {
            Orientation::Vertical => (b, a),
            Orientation::Horizontal => (a, b),
        };

        if !map.is_inbound(a, b) {
            continue;
        }

        let index = map.get_index_from_xy(a, b);
        map.tiles[index].kind = TileKind::Floor;
    }
}

enum Orientation {
    Vertical,
    Horizontal,
}
