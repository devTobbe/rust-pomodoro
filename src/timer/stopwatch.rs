use std::{
    thread,
    time::{Duration, SystemTime},
};

#[derive(Debug)]
pub struct Stopwatch {
    pub time: u64,
}

impl Stopwatch {
    pub fn new(time: u64) -> Self {
        Self { time }
    }

    pub fn start(&self) {
        let time_in_secs = self.time * 60;
        let duration = Duration::from_secs(time_in_secs);

        let start = SystemTime::now();
        loop {
            thread::sleep(duration);

            match start.elapsed() {
                Ok(elapsed) if elapsed > duration => {
                    return;
                }
                _ => (),
            }
        }
    }
}

