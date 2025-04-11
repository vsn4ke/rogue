use crate::player::input::player_input;
use crate::systems::Visibility;
use crate::systems::monster::MonsterAI;
use color_eyre::Result;
use color_eyre::eyre::bail;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{DefaultTerminal, Frame};
use specs::prelude::*;
use tabs::{DrawableComponent, MainTab};

pub mod tabs;

pub const TERMINAL_WIDTH: i32 = 80;
pub const TERMINAL_HEIGHT: i32 = 50;

pub struct App {
    pub state: AppState,
    pub world: World,
    pub tab: usize,
    pub main_tab: MainTab,
}

#[derive(PartialEq)]
pub enum AppState {
    Running,
    Closing,
    Paused,
}

impl Default for App {
    fn default() -> Self {
        App {
            state: AppState::Running,
            world: World::new(),
            tab: 0,
            main_tab: MainTab,
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state != AppState::Closing {
            if self.state == AppState::Running {
                self.run_systems();
                self.state = AppState::Paused
            } else {
                self.handle_events()?;
            }

            terminal.draw(|frame| {
                if let Err(e) = self.draw(frame) {
                    println!("unable to draw: {:?}", e);
                }
            })?;
        }

        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        let frame_size = frame.area();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), //future tabs bar
                Constraint::Min(2),
                Constraint::Length(2), //future nav bar
            ])
            .split(frame_size);

        match self.tab {
            0 => self.main_tab.draw(frame, chunks_main[1], &mut self.world)?,
            _ => bail!("unknow tab"),
        }

        Ok(())
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
