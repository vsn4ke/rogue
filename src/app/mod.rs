use crate::player::player_input;
use crate::systems::monster::MonsterAI;
use crate::systems::{MapIndexing, Visibility};
use crate::utils::Rng;
use drawable::bars::{BottomBar, TopBar};
use drawable::{MainTab, layout};
use logger::Logger;
use ratatui::DefaultTerminal;
use specs::prelude::*;

pub mod drawable;
pub mod logger;

pub const TERMINAL_WIDTH: i32 = 80;
pub const TERMINAL_HEIGHT: i32 = 50;

pub struct App {
    pub state: AppState,
    pub world: World,
    pub tab: usize,
    pub main_tab: MainTab,
    pub top_bar: TopBar,
    pub bottom_bar: BottomBar,
    pub rng: Rng,
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
            rng: Rng::random_seed(),
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) {
        while self.state != AppState::Closing {
            if self.state == AppState::Running {
                self.run_systems();
                self.state = AppState::Paused
            } else {
                player_input(&mut self);
            }

            if terminal
                .draw(|frame| layout::draw(&mut self, frame))
                .is_err()
            {
                let mut logger = self.world.fetch_mut::<Logger>();
                logger.add_entries("Err: frame skipped");
            }
        }
    }

    fn run_systems(&mut self) {
        let mut visibility = Visibility;
        visibility.run_now(&self.world);
        let mut map_indexing = MapIndexing;
        map_indexing.run_now(&self.world);
        let mut monster = MonsterAI;
        monster.run_now(&self.world);
        self.world.maintain();
    }
}
