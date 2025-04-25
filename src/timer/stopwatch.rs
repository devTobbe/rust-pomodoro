use std::{
    thread,
    time::{Duration, SystemTime},
};

// Stopwatch struct to keep track of time
#[derive(Debug)]
pub struct Stopwatch {
    pub time: u64,
}

impl Stopwatch {
    // Create a new stopwatch
    pub fn new(time: u64) -> Self {
        Self { time }
    }

    // Start the timer
    pub fn start(&self) {

        let time_in_secs = self.time * 60;
        let duration = Duration::from_secs(time_in_secs);

        let start = SystemTime::now();
        loop {
            thread::sleep(duration);

            // When time has elapsed, return
            match start.elapsed() {
                Ok(elapsed) if elapsed > duration => {
                    return;
                }
                _ => (),
            }
        }
    }
}

