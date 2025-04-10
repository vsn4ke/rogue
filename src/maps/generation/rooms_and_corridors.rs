use crate::components::Position;
use crate::maps::{Map, Orientation};
use crate::utils::{Rect, Rng};

pub fn rooms_and_corridors(max_rooms: i32, min_size: i32, max_size: i32) -> (Position, Map) {
    let mut map = Map::default();
    let mut rng = Rng::random_seed();

    for _ in 0..max_rooms {
        let w = rng.random_range(min_size..max_size);
        let h = rng.random_range(min_size..max_size);
        let room = Rect::new(
            rng.random_range(1..map.width - w - 1),
            rng.random_range(1..map.height - h - 1),
            w,
            h,
        );

        let mut ok = true;
        for other_room in map.rooms.iter() {
            if room.intersect(other_room) {
                ok = false
            }
        }

        if ok {
            map.apply_room(&room);

            if !map.rooms.is_empty() {
                let (new_x, new_y) = room.center();
                let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                let (middle_x, middle_y) = if rng.random_range(0..2) == 1 {
                    (new_x, prev_y)
                } else {
                    (prev_x, new_y)
                };

                map.draw_corridor(prev_x, new_x, middle_y, Orientation::Horizontal);
                map.draw_corridor(prev_y, new_y, middle_x, Orientation::Vertical);
            }
            map.rooms.push(room);
        }
    }

    (map.rooms[0].center_position(), map)
}
