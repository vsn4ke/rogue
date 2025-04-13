use app::App;
use color_eyre::Result;
use components::*;
use maps::rooms_and_corridors;
use ratatui::style::Color;
use specs::prelude::*;
use utils::Rng;

pub mod app;
pub mod components;
pub mod maps;
pub mod player;
pub mod systems;
pub mod utils;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let mut app = App::default();

    app.world.register::<Position>();
    app.world.register::<Renderable>();
    app.world.register::<Player>();
    app.world.register::<Viewshed>();
    app.world.register::<Monster>();
    app.world.register::<Name>();
    app.world.register::<BlockPath>();
    //app.world.register::<>();

    let (player_point, map) = rooms_and_corridors(20, 6, 10);
    //tmp: insert entities in rooms
    let mut rng = Rng::random_seed();
    for room in map.rooms.iter().skip(1) {
        let (glyph, name) = match rng.random_range(0..3) {
            1 => ('o', "Orc"),
            _ => ('g', "Goblin"),
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
        .build();

    app.world.insert(player_point);
    app.world.insert(map);
    app.run(terminal)?;
    ratatui::restore();
    Ok(())
}
