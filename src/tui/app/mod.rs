use crate::game::board::{Board, Point};

#[derive (Debug, Default)]
pub struct App {
    pub should_exit   : bool,
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
    pub fn tick(&self) {}

    pub fn exit(&mut self) {
        self.should_exit = true;
    }

    pub fn move_cursor(&mut self, direction : Direction) {
        match direction {
            Direction::Up => self.point.decrement_y(1),
            Direction::Down => self.point.increment_y(1),
            Direction::Left => self.point.decrement_x(1),
            Direction::Right => self.point.increment_x(1)
        };

    }

    pub fn place_mark(&mut self) {
        self.board.mark_coords(&self.point);
   }


}
