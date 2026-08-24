#[derive(Clone, Copy, Debug, Default)]

pub enum SlotState {
    P1,
    P2,
    #[default]
    Empty
}

impl std::fmt::Display for SlotState {

    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {

        let result = match self {
            SlotState::P1 => "Filled p1",
            SlotState::P2 => "Filled p2",
            SlotState::Empty => "Empty"
        };

        write!(f, "{}", result)
        
    }

}

#[derive(Clone, Copy, Debug, Default)]
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

    pub fn mark_slot(&mut self, new_state : SlotState) -> bool {

        match self.state {
            SlotState::P1 | SlotState::P2 => false, 
            SlotState::Empty => {
                self.state = new_state;
                self.life = 3;
                true
            }

        }
    }

    pub fn get_state(&self) -> SlotState{
        self.state
    }

    fn death(&mut self) {
        self.state = SlotState::Empty;
    }

}
