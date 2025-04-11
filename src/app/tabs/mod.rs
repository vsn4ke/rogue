use color_eyre::Result;
pub use main_tab::*;
use ratatui::{Frame, layout::Rect};
use specs::prelude::*;

pub mod main_tab;

pub trait DrawableComponent {
    fn draw(&self, frame: &mut Frame, rect: Rect, world: &mut World) -> Result<()>;
}
