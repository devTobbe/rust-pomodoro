mod cli;
mod config;
mod timer;

use clap::Parser;
use cli::{Cli, Command};
use config::Config;
use timer::Stopwatch;

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();

    match cli.command {
        // Enter if start is picked
        Command::Start {
            focus,
            break_,
            rounds,
        } => {
            // Read file if there exists one, let the specified value have
            // priority.
            let mut conf = config::readfile();

            // Check if the attributes have any value or not
            if focus.is_some() {
                conf.focus = focus.unwrap()
            }
            if break_.is_some() {
                conf.break_ = break_.unwrap()
            }
            if rounds.is_some() {
                conf.rounds = rounds.unwrap()
            }

            // Run a timer with the settings
            timer::run(conf.focus, conf.break_, conf.rounds);
        }
        Command::Focus { time } => {}
        Command::Break { time } => {}
        Command::Rounds { amount } => {}
    }
}
