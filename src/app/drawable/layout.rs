use super::Drawable;
use crate::app::App;
use ratatui::prelude::*;

pub const CAMERA_WIDTH: i32 = 60;
pub const CAMERA_HEIGHT: i32 = 30;

pub fn draw(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(CAMERA_WIDTH as u16),
            Constraint::Length(5),
        ])
        .split(frame.area());

    let chunks_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), //future tabs bar
            Constraint::Length(CAMERA_HEIGHT as u16 + 10),
            Constraint::Length(2), //future nav bar
        ])
        .split(layout[1]);

    app.top_bar.selected_tab = app.tab;

    app.top_bar.draw(frame, chunks_main[0], app);

    app.main_tab.draw(frame, chunks_main[1], app);

    app.bottom_bar.draw(frame, chunks_main[2], app);
}
