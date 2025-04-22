#[derive(Debug, Eq, PartialEq)]
pub enum SessionState {
    Focus,
    Break, 
}

#[derive(Debug)]
pub struct Session {
    pub rounds: u64,
    pub state : SessionState,
}

impl Session {
    pub fn new(rounds : u64) -> Self {
        Self {
            rounds,
            state: SessionState::Focus,
        }
    }

    pub fn advance(&mut self) -> bool {
        self.rounds -= 1;

        if self.rounds <= 0 {
            return false
        }
        self.flip_state();
        
        return true
    }
    
    pub fn flip_state(&mut self){
        if self.state == SessionState::Focus {
                self.state = SessionState::Break;
                return
        }
        else {
            self.state = SessionState::Focus;
            return
        }
    }

}
