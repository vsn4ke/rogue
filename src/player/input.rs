use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::{App, AppState};

use super::try_move_player;

pub fn player_input(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.state = AppState::Closing;
                    return Ok(());
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
    Ok(())
}
