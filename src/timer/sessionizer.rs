enum session_state {
    focus,
    break_
}

#[derive(Debug)]
struct Session {
    rounds: u32,
    state : session_state,
}

impl Session {
    pub fn new(rounds : u32) -> Self {
        Self {
            rounds,
            session_state.focus,
        }
    }

    pub fn advance(&self) -> bool {
        Self.rounds -= 1;

        if Self.rounds <= 0 {
            return false
        }
        flip_state();
        
        return true
    }
    
    fn flip_state(&self){
        if Self.state == session_state.focus {
                Self.state = session_state.break_;
                return
        }
        else {
            Self.state = session_state.focus;
            return
        }

        return
    }
}
