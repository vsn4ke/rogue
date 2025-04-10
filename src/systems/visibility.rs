use specs::prelude::*;

use crate::{
    components::{Player, Position, Viewshed},
    maps::Map,
    utils::{Point, get_line, max, min},
};

pub struct Visibility;

impl<'a> System<'a> for Visibility {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;
        for (entity, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if !viewshed.dirty {
                continue;
            }

            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = fov(Point::from_position(*pos), viewshed.range, &map);

            if player.get(entity).is_none() {
                continue;
            }

            map.clear_visibility();

            for visible in viewshed.visible_tiles.iter() {
                let index = map.xy_to_index(visible.x, visible.y);
                map.tiles[index].revealed = true;
                map.tiles[index].visible = true;
            }
        }
    }
}

fn fov(center: Point, range: i32, map: &Map) -> Vec<Point> {
    let mut visible_tile = Vec::<Point>::new();

    let left_bound = center.y - range;
    let right_bound = center.y + range;
    let top_bound = center.x + range;
    let bottom_bound = center.x - range;

    for x in max(bottom_bound, 0)..min(top_bound, map.width) {
        add_visible_tile(
            map,
            get_line(center, Point { x, y: left_bound }),
            &mut visible_tile,
        );

        add_visible_tile(
            map,
            get_line(center, Point { x, y: right_bound }),
            &mut visible_tile,
        );
    }

    for y in max(left_bound + 1, 0)..min(right_bound - 1, map.height) {
        add_visible_tile(
            map,
            get_line(center, Point { x: bottom_bound, y }),
            &mut visible_tile,
        );

        add_visible_tile(
            map,
            get_line(center, Point { x: top_bound, y }),
            &mut visible_tile,
        );
    }

    visible_tile
}

fn add_visible_tile(map: &Map, tile_list: Vec<Point>, list_to_update: &mut Vec<Point>) {
    for tile in tile_list.iter() {
        let index = map.point_to_index(*tile);
        if !map.is_point_inbound(*tile) {
            continue;
        }

        list_to_update.push(*tile);

        if map.tiles[index].is_opaque() {
            break;
        }
    }
}
