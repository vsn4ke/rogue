use app::App;
use color_eyre::Result;
use components::*;
use maps::rooms_and_corridors;
use ratatui::style::Color;
use specs::prelude::*;

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
    //app.world.register::<>();

    let (player_position, map) = rooms_and_corridors(20, 6, 10);
    //tmp: insert entities in rooms
    for room in map.rooms.iter().skip(1) {
        app.world
            .create_entity()
            .with(room.center_position())
            .with(Renderable {
                glyph: 'g',
                fg: Color::Red,
                ..Default::default()
            })
            .with(Viewshed::new(7))
            .with(Monster)
            .build();
    }

    app.world
        .create_entity()
        .with(Renderable {
            glyph: '@',
            fg: Color::Black,
            bg: Color::Red,
        })
        .with(Position::from_point(player_position))
        .with(Player)
        .with(Viewshed::new(8))
        .build();

    app.world.insert(player_position);
    app.world.insert(map);
    app.run(terminal)?;
    ratatui::restore();
    Ok(())
}
