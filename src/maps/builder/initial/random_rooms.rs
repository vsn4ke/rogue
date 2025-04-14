use crate::maps::{InitialBuilder, Map, TileKind};
use crate::utils::{Point, Rect, Rng};

pub struct RandomRooms {
    max_rooms: i32,
    min_size: i32,
    max_size: i32,
}
impl RandomRooms {
    pub fn new(max_rooms: i32, min_size: i32, max_size: i32) -> Box<RandomRooms> {
        Box::new(RandomRooms {
            max_rooms,
            min_size,
            max_size,
        })
    }
}

impl InitialBuilder for RandomRooms {
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
            for y in room.y1 + 1..=room.y2 {
                for x in room.x1 + 1..=room.x2 {
                    let index = map.get_index_from_xy(x, y);
                    map.tiles[index].kind = TileKind::Floor;
                }
            }
        }

        map.starter_point = if rooms.is_empty() {
            Point::default()
        } else {
            rooms[0].center_point()
        };

        map.rooms = rooms;
    }
}
