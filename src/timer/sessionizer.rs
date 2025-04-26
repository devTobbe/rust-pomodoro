// Saves the current state of a session
#[derive(Debug, Eq, PartialEq)]
pub enum SessionState {
    Focus,
    Break, 
}

// Keeps track of session information
#[derive(Debug)]
pub struct Session {
    pub rounds: u64,
    pub state : SessionState,
}

impl Session {
    // Creates and retruns a new session object
    pub fn new(rounds : u64) -> Self {
        Self {
            rounds,
            state: SessionState::Focus,
        }
    }

    // Advances the status of the session
    pub fn advance(&mut self) -> bool {
        self.rounds -= 1;

        if self.rounds <= 0 {
            return false
        }
        self.flip_state();
        
        return true
    }
    
    // TODO: Refactor to toggle // Toggles the state of the session
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
