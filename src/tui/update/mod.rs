use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::game::board::Player;

use crate::tui::app::{App, Direction};
pub fn handle_update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.exit(),

        // Player 1 movement (Arrow keys)
        KeyCode::Up    => app.move_cursor(Direction::Up, &Player::P1),
        KeyCode::Down  => app.move_cursor(Direction::Down, &Player::P1),
        KeyCode::Left  => app.move_cursor(Direction::Left, &Player::P1),
        KeyCode::Right => app.move_cursor(Direction::Right, &Player::P1),

        // Player 2 movement (WASD)
        KeyCode::Char('w') => app.move_cursor(Direction::Up, &Player::P2),
        KeyCode::Char('s') => app.move_cursor(Direction::Down, &Player::P2),
        KeyCode::Char('a') => app.move_cursor(Direction::Left, &Player::P2),
        KeyCode::Char('d') => app.move_cursor(Direction::Right, &Player::P2),

        // Mark placement
        KeyCode::Enter     => app.place_mark(&Player::P1),
        KeyCode::Char(' ') => app.place_mark(&Player::P2),

        _ => {}
    };
}
