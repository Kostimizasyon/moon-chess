use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::{App, Direction};

pub fn handle_update(app : &mut App, key_event : KeyEvent) {
    match key_event.code {

        KeyCode::Esc => app.exit(),
        KeyCode::Char('k') | KeyCode::Up => app.move_cursor(Direction::Up),
        KeyCode::Char('j') | KeyCode::Down => app.move_cursor(Direction::Down),
        KeyCode::Char('h') | KeyCode::Left => app.move_cursor(Direction::Left),
        KeyCode::Char('l') | KeyCode::Right => app.move_cursor(Direction::Right),
        KeyCode::Char(' ') => app.place_mark(),
        _ => {}

    };
}
