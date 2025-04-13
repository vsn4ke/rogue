use color_eyre::Result;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders},
};
use specs::prelude::*;

use crate::{
    components::{Position, Renderable},
    maps::Map,
    utils::Point,
};

use super::Drawable;

pub struct MainTab;

impl Drawable for MainTab {
    fn draw(&self, frame: &mut Frame, rect: Rect, world: &mut World) -> Result<()> {
        const CAMERA_WIDTH: i32 = 60;
        const CAMERA_HEIGHT: i32 = 30;

        let layout_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(CAMERA_WIDTH as u16),
                Constraint::Length(30),
            ])
            .split(rect);
        let layout_vertical_0 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(CAMERA_HEIGHT as u16), Constraint::Min(2)])
            .split(layout_horizontal[0]);
        let layout_vertical_1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(15), Constraint::Min(2)])
            .split(layout_horizontal[1]);

        frame.render_widget(
            camera(world, CAMERA_WIDTH, CAMERA_HEIGHT),
            layout_vertical_0[0],
        );

        frame.render_widget(
            Block::new()
                .borders(Borders::TOP | Borders::LEFT)
                .title(" Top Left Block "),
            layout_vertical_0[0],
        );

        let bottom_left_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            horizontal_top: symbols::line::NORMAL.horizontal,
            ..symbols::border::PLAIN
        };

        frame.render_widget(
            Block::new()
                .border_set(bottom_left_border_set)
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                .title(" Bottom Left Block "),
            layout_vertical_0[1],
        );

        let collapsed_left_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.horizontal_down,
            ..symbols::border::PLAIN
        };

        frame.render_widget(
            Block::new()
                .border_set(collapsed_left_border_set)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .title(" Top Right Block "),
            layout_vertical_1[0],
        );

        let collapsed_top_and_left_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            bottom_left: symbols::line::NORMAL.horizontal_up,
            ..symbols::border::PLAIN
        };
        frame.render_widget(
            Block::new()
                .border_set(collapsed_top_and_left_border_set)
                .borders(Borders::ALL)
                .title(" Bottom Right Block "),
            layout_vertical_1[1],
        );

        Ok(())
    }
}

fn camera(world: &mut World, width: i32, height: i32) -> Text<'_> {
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
