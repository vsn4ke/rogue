use color_eyre::Result;
pub use main_tab::*;
use ratatui::{Frame, layout::Rect};
use specs::prelude::*;

pub mod bars;
pub mod main_tab;

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, rect: Rect, world: &mut World) -> Result<()>;
}
