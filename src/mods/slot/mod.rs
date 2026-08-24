#[derive(Clone, Copy, PartialEq)]
pub enum SlotState {
    P1,
    P2,
    Empty
}

#[derive(Clone, Copy)]
pub struct Slot {

    state : SlotState, // what the current state of the slot is
    life : usize,      // the life of the current slot

}

impl Slot {

    pub fn new() -> Self {
        let life = 0;

        let state = SlotState::Empty;

        Self{state, life}
    }

    pub fn tick_life(&mut self) {
       
        match self.state {

            SlotState::P1 => self.life -= 1,
            SlotState::P2 => self.life -= 1,
            SlotState::Empty => {}

        }

        if self.life == 0 {
            self.death();
        }

    }

    pub fn get_state(&self) -> SlotState {
        self.state
    }

    pub fn mark_slot(&mut self, new_state : SlotState) -> isize {

        let mut to_ret = -1;

        match self.state {

            SlotState::P1 => println!("Cannot mark an already marked slot!"),
            SlotState::P2 => println!("Cannot mark an already marked slot!"),
            SlotState::Empty => {
                self.state = new_state;                                  
                self.life = 7;
                to_ret = 0;
            }

        }
        
        to_ret 

    }

    fn death(&mut self) {
        self.state = SlotState::Empty;
    }

}
