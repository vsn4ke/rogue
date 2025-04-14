use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

use crate::app::{App, AppState};

use super::try_move_player;

pub fn player_input(app: &mut App) {
    if event::poll(Duration::from_millis(200)).is_ok() {
        match event::read().unwrap() {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.state = AppState::Closing;
                }

                KeyCode::Up => try_move_player(0, -1, app),
                KeyCode::Down => try_move_player(0, 1, app),
                KeyCode::Left => try_move_player(-1, 0, app),
                KeyCode::Right => try_move_player(1, 0, app),
                _ => {}
            },
            _ => {}
        }
    }
}
