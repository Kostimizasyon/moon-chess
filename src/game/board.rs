use crate::game::slot::{self, Slot, SlotState};

// shoudl probably merge slotstate and palyer into one enum

#[derive (Debug, Default)]
pub struct Point {
    x : usize,
    y : usize
}

impl Point {
    pub fn new(x : usize, y : usize) -> Self {
        Self{x, y}
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn increment_x(&mut self, val : usize) {
        // clamp val
        if self.x == 2 {
            self.x = 0;
            return;
        }
        self.x += val;
    }
    
    pub fn decrement_x(&mut self, val : usize) {
        // clamp val
        if self.x == 0 {
            self.x = 2;
            return;
        }
        self.x -= val;
    }

    pub fn increment_y(&mut self, val : usize) {
        // clamp val
        if self.y == 2 {
            self.y = 0;
            return;
        }
        self.y += val;
    }
    
    pub fn decrement_y(&mut self, val : usize) {
        // clamp val
        if self.y == 0 {
            self.y = 2;
            return;
        }
        self.y -= val;
    }
}

#[derive(Clone, Copy, Debug, Default)]  
pub enum Player {
    #[default]
    P1,
    P2
}

impl Player {

    pub fn to_slotstate(&self) -> slot::SlotState {
        match self {
            Player::P1 => slot::SlotState::P1,
            Player::P2 => slot::SlotState::P2,
        }
    }



}

impl std::fmt::Display for Player {

    fn fmt(&self,f : &mut std::fmt::Formatter ) -> std::fmt::Result {

        let s = match self {
            Player::P1 => "Player 1",
            Player::P2 => "Player 2"
        };

        write!(f, "{}", s)
    }

}

#[derive(Debug)]
pub struct Board {
    pub total_moves  : usize,
    pub curr_turn    : Player,
    pub display_life : bool,
    pub board        : Vec<Vec<slot::Slot>>
}

impl std::default::Default for Board {
    fn default() -> Self {
        let board = vec![vec![slot::Slot::new(); 3]; 3];
        let curr_turn = Player::P1;
        let total_moves = 0;
        let display_life = false;

        Self{ total_moves, curr_turn ,display_life, board }
    }
}

impl Board {

    pub fn new(display_life : Option<bool>) -> Self {
        
        let board = vec![vec![slot::Slot::new(); 3]; 3];
        let curr_turn = Player::P1;
        let total_moves = 0;
        let display_life = display_life.unwrap_or(false);

        Self{ total_moves, curr_turn ,display_life, board }

    }

    pub fn mark_coords(&mut self, point : &Point) {
        println!("{}, {}", point.to_tuple().1, point.to_tuple().0);

        println!("{}", self.curr_turn.to_slotstate());

        self.at_point(point).mark_slot(self.curr_turn.to_slotstate());
        self.next_turn();
    }

    fn next_turn(&mut self) {
        
        self.total_moves += 1;

        match self.curr_turn {
            Player::P1 => self.curr_turn = Player::P2,          
            Player::P2 => self.curr_turn = Player::P1,
        }

        self.board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                cell.tick_life();
            });
        });

        if self.total_moves >= 5 {
            Self::calc_win(self);    
        } 

    }

    pub fn get_slot_state(&self, point : &Point) -> SlotState {
        self.at_point(point).get_state()
    }

    pub fn reset(&mut self) {
        *self = Self::new(None);
    }

    fn calc_win(&mut self) {

    // coudl do some cool match shit so ill wait   

    // // print winnter
    //     println!("{} wins!", self.curr_turn);
    // // restart game
    //     Self::reset(self);

    }

    fn at_point(&self, point : &Point) -> Slot {
        let (x, y) = point.to_tuple();
        self.board[y][x]
    }

}
