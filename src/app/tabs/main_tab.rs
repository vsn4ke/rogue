use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span, Text},
};
use specs::prelude::*;

use crate::{
    components::{Position, Renderable},
    maps::Map,
};

use super::DrawableComponent;

pub struct MainTab;

impl DrawableComponent for MainTab {
    fn draw(&self, frame: &mut Frame, rect: Rect, world: &mut World) -> Result<()> {
        let positions = world.read_storage::<Position>();
        let renderables = world.read_storage::<Renderable>();
        let map = world.fetch::<Map>();

        let mut lines = Vec::new();
        for _ in 0..map.height {
            lines.push(Line::from(vec![
                Span::default().content(" ");
                map.width as usize
            ]))
        }

        let (mut x, mut y) = (0, 0);
        for (index, tile) in map.tiles.iter().enumerate() {
            if map.tiles[index].revealed {
                lines[y as usize].spans[x as usize] = tile.renderable().draw();
            }

            x += 1;
            if x > map.width - 1 {
                x = 0;
                y += 1;
            }
        }

        for (pos, render) in (&positions, &renderables).join() {
            let index = map.position_to_index(*pos);
            if !map.tiles[index].visible {
                continue;
            }
            lines[pos.y as usize].spans[pos.x as usize] = render.draw();
        }

        let text = Text::from(lines);
        frame.render_widget(text, rect);
        Ok(())
    }
}
