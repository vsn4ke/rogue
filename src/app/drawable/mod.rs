use super::App;
use color_eyre::Result;
pub use main_tab::*;
use ratatui::{Frame, layout::Rect};

pub mod bars;
pub mod main_tab;

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, rect: Rect, app: &App) -> Result<()>;
}
