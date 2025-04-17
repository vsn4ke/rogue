use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders},
};
use specs::prelude::*;

use crate::{
    app::{App, logger::Logger},
    components::{Position, Renderable},
    maps::Map,
    utils::Point,
};

use super::{
    Drawable,
    layout::{CAMERA_HEIGHT, CAMERA_WIDTH},
};

pub struct MainTab;

impl Drawable for MainTab {
    fn draw(&self, frame: &mut Frame, rect: Rect, app: &App) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(CAMERA_HEIGHT as u16),
                Constraint::Length(10),
            ])
            .split(rect);

        frame.render_widget(camera(&app.world, CAMERA_WIDTH, CAMERA_HEIGHT), layout[0]);

        frame.render_widget(
            Block::new().borders(Borders::ALL).title(" Top Block "),
            layout[0],
        );

        frame.render_widget(logger(&app.world), layout[1]);
        frame.render_widget(
            Block::new().borders(Borders::ALL).title(" Bottom Block "),
            layout[1],
        );
    }
}

fn camera(world: &World, width: i32, height: i32) -> Text<'_> {
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();
    let map = world.fetch::<Map>();
    let player_point = world.fetch::<Point>();

    let dx = player_point.x - width / 2;
    let dy = player_point.y - height / 2;

    let mut lines = Vec::new();
    for y in 0..height {
        lines.push(Line::from(vec![
            Span::default().content(" ");
            width as usize
        ]));

        if y == 0 {
            continue;
        }

        for x in 1..width {
            let cx = x + dx;
            let cy = y + dy;
            let index = map.get_index_from_xy(cx, cy);

            if !map.is_inbound(cx, cy) || !map.tiles[index].revealed {
                continue;
            }

            lines[y as usize].spans[x as usize] = map.tiles[index].renderable().draw();
        }
    }

    for (pos, render) in (&positions, &renderables).join() {
        let index = map.get_index_from_position(*pos);
        if !map.tiles[index].visible {
            continue;
        }

        let x = (pos.x - dx) as usize;
        let y = (pos.y - dy) as usize;

        lines[y].spans[x] = render.draw();
    }

    Text::from(lines)
}

fn logger(world: &World) -> Text<'static> {
    let logger = world.fetch::<Logger>();
    let mut lines = Vec::new();

    for log in logger.get_last_entries(8).iter() {
        lines.push(Line::from("  ".to_owned() + log));
    }

    Text::from(lines)
}
