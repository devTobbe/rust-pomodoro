use std::{
    io::{self, Write},
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
            let elapsed = start.elapsed().unwrap_or_default();

            if elapsed >= duration {
                break;
            }

            let remaining = duration - elapsed;
            let mins = remaining.as_secs() / 60;
            let secs = remaining.as_secs() % 60;

            // Print remaining time, overwrite previous line
            print!("\rTime left: {:02}:{:02}", mins, secs);
            io::stdout().flush().unwrap();

            thread::sleep(Duration::from_secs(1));
        }
        println!("\n")
    }
}
