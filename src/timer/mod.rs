pub mod sessionizer;
pub mod stopwatch;

pub use sessionizer::Session;
pub use sessionizer::SessionState;
pub use stopwatch::Stopwatch;


pub fn run(focus: u64, break_: u64, rounds: u64) {
    let mut session = Session::new(rounds);

    loop {
        println!("🦀🍅: Time for {} in round {}!", session.get_state_as_string(), session.rounds.to_string());
        match session.state {
            SessionState::Focus => {
                let sw = Stopwatch::new(focus);
                sw.start();
                session.flip_state();
            }
            SessionState::Break => {
                if session.check_done() {
                    println!("🦀🍅: You are done! Good job!");
                    break;
                }
                let sw = Stopwatch::new(break_);
                sw.start();
                session.advance();
            }
        }
    }
}
