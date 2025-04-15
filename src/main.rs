use app::{App, logger::Logger};
use components::*;
use maps::finalized::rooms_and_corridors;
use ratatui::style::Color;
use specs::prelude::*;
use utils::Rng;

pub mod app;
pub mod components;
pub mod maps;
pub mod player;
pub mod systems;
pub mod utils;

fn main() {
    let terminal = ratatui::init();
    let mut app = App::default();

    app.world.register::<Position>();
    app.world.register::<Renderable>();
    app.world.register::<Player>();
    app.world.register::<Viewshed>();
    app.world.register::<Monster>();
    app.world.register::<Name>();
    app.world.register::<BlockPath>();
    app.world.register::<CombatStats>();
    //app.world.register::<>();

    let map = rooms_and_corridors();
    let player_point = map.starter_point;

    //tmp: insert entities in rooms
    let mut rng = Rng::random_seed();

    for room in map.rooms.iter().skip(1) {
        let (glyph, name, stats) = match rng.random_range(0..3) {
            1 => ('o', "Orc", CombatStats::new(20, 2, 4)),
            _ => ('g', "Goblin", CombatStats::new(18, 1, 3)),
        };

        app.world
            .create_entity()
            .with(room.center_position())
            .with(Renderable {
                glyph,
                fg: Color::Red,
                ..Default::default()
            })
            .with(Viewshed::new(7))
            .with(Monster)
            .with(Name::new(name))
            .with(BlockPath)
            .with(stats)
            .build();
    }

    app.world
        .create_entity()
        .with(Renderable {
            glyph: '@',
            fg: Color::Black,
            bg: Color::Red,
        })
        .with(Position::from_point(player_point))
        .with(Player)
        .with(Name::new("Player"))
        .with(Viewshed::new(8))
        .with(CombatStats::new(50, 2, 6))
        .build();

    app.world.insert(Logger::default());
    app.world.insert(player_point);
    app.world.insert(map);
    app.run(terminal);
    ratatui::restore();
}
