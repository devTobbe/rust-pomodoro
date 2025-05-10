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
    pub state: SessionState,
}

impl Session {
    // Creates and retruns a new session object
    pub fn new(rounds: u64) -> Self {
        Self {
            rounds,
            state: SessionState::Focus,
        }
    }

    // Advances the status of the session
    pub fn advance(&mut self) {
        self.rounds -= 1;
        self.flip_state();
    }

    // Checks whether or not there are any rounds left
    pub fn check_done(&mut self) -> bool {
        if self.rounds <= 0 {
            return true;
        }
        return false;
    }

    // Flips the state of the session
    pub fn flip_state(&mut self) {
        if self.state == SessionState::Focus {
            self.state = SessionState::Break;
            return;
        } else {
            self.state = SessionState::Focus;
            return;
        }
    }

    // Returns the current state as a string
    pub fn get_state_as_string(&self) -> String {
        match self.state {
            SessionState::Focus => return String::from("Focus"),
            SessionState::Break => return String::from("Break"),
        }
    }
}
