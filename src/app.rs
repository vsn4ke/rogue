use crate::components::{Position, Renderable};
use crate::maps::Map;
use crate::player::input::player_input;
use crate::systems::Visibility;
use crate::systems::monster::MonsterAI;
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    text::{Line, Span, Text},
};
use specs::prelude::*;

pub const TERMINAL_WIDTH: i32 = 80;
pub const TERMINAL_HEIGHT: i32 = 50;

pub struct App {
    pub state: AppState,
    pub world: World,
}

#[derive(PartialEq)]
pub enum AppState {
    Running,
    Closing,
}

impl Default for App {
    fn default() -> Self {
        App {
            state: AppState::Running,
            world: World::new(),
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| self.render(frame))?;
            self.run_systems();
            self.handle_events()?;
        }
        match self.state {
            AppState::Closing => Ok(()),
            AppState::Running => unreachable!(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();
        let map = self.world.fetch::<Map>();

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
        frame.render_widget(text, frame.area());
    }

    pub fn handle_events(&mut self) -> Result<()> {
        player_input(self)?;
        Ok(())
    }

    fn run_systems(&mut self) {
        let mut visibility = Visibility;
        visibility.run_now(&self.world);
        let mut monster = MonsterAI;
        monster.run_now(&self.world);
        self.world.maintain();
    }
}
