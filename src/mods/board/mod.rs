use crate::mods::slot::SlotState;

use super::slot;

// shoudl probably merge slotstate and palyer into one enum

#[derive(Clone, Copy)]  
pub enum Player {
    P1,
    P2
}

impl Player {

    fn to_slotstate(&self) -> SlotState {

        let result = match self {
            Player::P1 => SlotState::P1,
            Player::P2 => SlotState::P2,
        };

        result

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

pub struct Board {
    total_moves : usize,
    curr_turn : Player,
    display_life : bool,
    board : Vec<Vec<slot::Slot>>
}

impl Board {
 
    pub fn new(display_life : Option<bool>) -> Self {
        
        let board = vec![vec![slot::Slot::new(); 3]; 3];
        let curr_turn = Player::P1;
        let total_moves = 0;
        let display_life = display_life.unwrap_or(false);

        Self{ total_moves, curr_turn ,display_life, board }

    }

    // maybe create a struct for coords
    pub fn mark_coords(&mut self, x : usize, y : usize) -> isize {
        // validate input somehow
        self.board[y][x].mark_slot(self.curr_turn.to_slotstate())
    }

    pub fn next_turn(&mut self) {
        
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

    pub fn reset(&mut self) {
        *self = Self::new(None);
    }

    fn calc_win(&mut self) {

        // Slots carrying their point info would make this a lot simpler

        let player_win = {

            let mut x_vec = vec![];
            let mut y_vec = vec![];

            for (y, vec) in self.board.iter().enumerate() {
                    for (x, slot) in vec.iter().enumerate() {
                     if slot.get_state() == self.curr_turn.to_slotstate() {
                        y_vec.push(x);
                        x_vec.push(y);
                    }
                }
            }

            x_vec.sort_unstable();
            y_vec.sort_unstable();

            (x_vec[1] - x_vec[0]) == (x_vec[2] - x_vec[1]) && x_vec[0] != x_vec[2]
                                            ||
            (y_vec[1] - y_vec[0]) == (y_vec[2] - y_vec[1]) && y_vec[0] != y_vec[2]
        };

        if player_win {
            println!("{} wins!", self.curr_turn);
            Self::reset(self);
        }
   
    }

}


