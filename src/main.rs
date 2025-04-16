use app::{App, logger::Logger};
use components::*;
use maps::finalized::rooms_and_corridors;
use spawner::{build_player_entity, spawn_monsters};
use specs::prelude::*;
use utils::Rng;

pub mod app;
pub mod colors;
pub mod components;
pub mod maps;
pub mod player;
pub mod spawner;
pub mod systems;
pub mod utils;

fn main() {
    let terminal = ratatui::init();
    let mut app = App {
        map_rng: Rng::new(1),
        ..Default::default()
    };

    app.world.register::<Position>();
    app.world.register::<Renderable>();
    app.world.register::<Player>();
    app.world.register::<Viewshed>();
    app.world.register::<Monster>();
    app.world.register::<Name>();
    app.world.register::<BlockPath>();
    app.world.register::<CombatStats>();
    //app.world.register::<>();

    //-- todo move away from main
    let map = rooms_and_corridors(&mut app.map_rng);
    spawn_monsters(&mut app.rng, &mut app.world, &map);
    //-- todo

    let player_point = map.starter_point;
    build_player_entity(&mut app.world, player_point);

    app.world.insert(Logger::default());
    app.world.insert(player_point);
    app.world.insert(map);
    app.run(terminal);
    ratatui::restore();
}
