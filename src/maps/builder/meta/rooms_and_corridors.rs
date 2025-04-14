use crate::maps::{Map, MetaBuilder, TileKind};
use crate::utils::{Point, Rect, Rng, order_value};

pub struct RoomsAndCorridors {
    max_rooms: i32,
    min_size: i32,
    max_size: i32,
}
impl RoomsAndCorridors {
    pub fn new(max_rooms: i32, min_size: i32, max_size: i32) -> Box<RoomsAndCorridors> {
        Box::new(RoomsAndCorridors {
            max_rooms,
            min_size,
            max_size,
        })
    }
}

impl MetaBuilder for RoomsAndCorridors {
    fn draw(&self, map: &mut Map) {
        let mut rng = Rng::random_seed();

        let mut rooms = Vec::new();

        for _ in 0..self.max_rooms {
            let w = rng.random_range(self.min_size..self.max_size);
            let h = rng.random_range(self.min_size..self.max_size);
            let room = Rect::new(
                rng.random_range(1..map.width - w - 1),
                rng.random_range(1..map.height - h - 1),
                w,
                h,
            );

            let mut ok = true;

            for other_room in rooms.iter() {
                if room.intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                rooms.push(room);
            }
        }

        for room in rooms.iter() {
            insert_room(map, room);

            let (new_x, new_y) = room.center();
            let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

            let (middle_x, middle_y) = if rng.random_range(0..2) == 1 {
                (new_x, prev_y)
            } else {
                (prev_x, new_y)
            };

            draw_corridor(map, prev_x, new_x, middle_y, Orientation::Horizontal);
            draw_corridor(map, prev_y, new_y, middle_x, Orientation::Vertical);
        }

        map.starter_point = if rooms.is_empty() {
            Point::default()
        } else {
            rooms[0].center_point()
        };

        map.rooms = Some(rooms);
    }
}

fn insert_room(map: &mut Map, room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            let index = map.get_index_from_xy(x, y);
            map.tiles[index].kind = TileKind::Floor;
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
