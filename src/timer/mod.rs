pub mod sessionizer;
pub mod stopwatch;

pub use sessionizer::Session;
pub use sessionizer::SessionState;
pub use stopwatch::Stopwatch;

pub fn run(focus: u64, break_: u64, rounds: u64) {
    let mut session = Session::new(rounds);

    for _ in 0..rounds {
        match session.state {
            SessionState::Focus => {
                let sw = Stopwatch::new(focus);
                sw.start();
                if session.advance() {
                    break;
                }
            }
            SessionState::Break => {
                let sw = Stopwatch::new(break_);
                sw.start();
                session.flip_state();
            }
        }
    }
}
