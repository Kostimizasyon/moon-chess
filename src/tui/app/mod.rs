use std::{thread::sleep, time::Duration};

use crate::game::board::{Board, Player, Point};

#[derive (Debug, Default)]
pub struct App {
    pub should_exit   : bool,
    pub pending_reset : bool,
    pub point         : Point,
    pub board         : Board
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}


impl App {

    pub fn new() -> Self {
        Self::default()
    }

    // tick the terminal
    pub fn tick(&self) {
        println!("Tic toc");
    }

    pub fn exit(&mut self) {
        self.should_exit = true;
    }

pub fn move_cursor(&mut self, direction: Direction, input_info: &Player) {
    if self.board.curr_turn != *input_info {
        return;
    }

    match direction {
        Direction::Up => self.point.decrement_y(1),
        Direction::Down => self.point.increment_y(1),
        Direction::Left => self.point.decrement_x(1),
        Direction::Right => self.point.increment_x(1),
    };
}

// hacky way also kidna funny so ill keep it for now
pub fn place_mark(&mut self, input_info: &Player) {
    if self.board.curr_turn != *input_info {
        return;
    }
    if self.pending_reset {
        self.pending_reset = false;
        sleep(Duration::from_secs(1));
        self.point.reset();
        self.board.reset();
        return;
    }

    self.board.mark_coords(&self.point);

    if self.board.did_win {
        self.pending_reset = true;
    }
}
}
