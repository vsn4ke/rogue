use crate::app::{App, AppState};
use crate::components::{Player, Position};
use crate::maps::{Map, TileKind};
use crate::utils::{Point, clamp};
use specs::prelude::*;

pub fn try_move_player(dx: i32, dy: i32, app: &mut App) {
    let mut pos = app.world.write_storage::<Position>();
    let player = app.world.read_storage::<Player>();
    let map = app.world.fetch::<Map>();
    let mut player_position = app.world.write_resource::<Point>();

    for (_, pos) in (&player, &mut pos).join() {
        let index = map.xy_to_index(pos.x + dx, pos.y + dy);
        if map.tiles[index].kind != TileKind::Wall {
            pos.x = clamp(0, map.width - 1, pos.x + dx);
            pos.y = clamp(0, map.height - 1, pos.y + dy);

            player_position.x = pos.x;
            player_position.y = pos.y;

            app.state = AppState::Running;
        }
    }
}
