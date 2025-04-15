use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

use crate::app::logger::Logger;
use crate::app::{App, AppState};
use crate::components::{CombatStats, Player, Position, Viewshed};
use crate::maps::Map;
use crate::utils::{Point, clamp};
use specs::prelude::*;

pub fn player_input(app: &mut App) {
    if event::poll(Duration::from_millis(200)).is_ok() {
        match event::read().unwrap() {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.state = AppState::Closing;
                }

                KeyCode::Up => try_move_player(0, -1, app),
                KeyCode::Down => try_move_player(0, 1, app),
                KeyCode::Left => try_move_player(-1, 0, app),
                KeyCode::Right => try_move_player(1, 0, app),
                _ => {}
            },
            _ => {}
        }
    }
}

fn try_move_player(dx: i32, dy: i32, app: &mut App) {
    let mut pos = app.world.write_storage::<Position>();
    let player = app.world.read_storage::<Player>();
    let map = app.world.fetch::<Map>();
    let mut player_position = app.world.write_resource::<Point>();
    let mut viewsheds = app.world.write_storage::<Viewshed>();
    let combat_stats = app.world.read_storage::<CombatStats>();
    let mut logger = app.world.fetch_mut::<Logger>();

    for (_, pos, viewshed) in (&player, &mut pos, &mut viewsheds).join() {
        let index = map.get_index_from_xy(pos.x + dx, pos.y + dy);

        for potential_target in map.content[index].iter() {
            if combat_stats.get(*potential_target).is_some() {
                logger.add_entries("Attack!");
                return;
            }
        }

        if map.tiles[index].is_blocked() {
            continue;
        }
        pos.x = clamp(0, map.width - 1, pos.x + dx);
        pos.y = clamp(0, map.height - 1, pos.y + dy);

        player_position.x = pos.x;
        player_position.y = pos.y;

        viewshed.dirty = true;
        app.state = AppState::Running;
    }
}
