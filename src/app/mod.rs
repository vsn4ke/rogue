use crate::player::input::player_input;
use crate::systems::Visibility;
use crate::systems::monster::MonsterAI;
use color_eyre::Result;
use color_eyre::eyre::bail;
use drawable::bars::{BottomBar, TopBar};
use drawable::{Drawable, MainTab};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{DefaultTerminal, Frame};
use specs::prelude::*;

pub mod drawable;

pub const TERMINAL_WIDTH: i32 = 80;
pub const TERMINAL_HEIGHT: i32 = 50;

pub struct App {
    pub state: AppState,
    pub world: World,
    pub tab: usize,
    pub main_tab: MainTab,
    pub top_bar: TopBar,
    pub bottom_bar: BottomBar,
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
            tab: 1,
            main_tab: MainTab,
            top_bar: TopBar {
                tab_list: vec!["main tab".into(), "second tab".into(), "third tab".into()],
                selected_tab: 1,
            },
            bottom_bar: BottomBar,
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

        self.top_bar.selected_tab = self.tab;

        self.top_bar.draw(frame, chunks_main[0], &mut self.world)?;

        match self.tab {
            1 => self.main_tab.draw(frame, chunks_main[1], &mut self.world)?,
            _ => bail!("unknow tab"),
        }

        self.bottom_bar
            .draw(frame, chunks_main[2], &mut self.world)?;

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
