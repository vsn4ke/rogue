use super::App;
pub use main_tab::*;
use ratatui::{Frame, layout::Rect};

pub mod bars;
pub mod layout;
pub mod main_tab;

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, rect: Rect, app: &App);
}
