mod cli;
mod config;
mod timer;

use clap::Parser;
use cli::{Cli, Command};
use config::{Config}

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();

    match cli.command {
        Command::Start {focus, break_, rounds} => {}
        Command::Focus {time} => {}
        Command::Break {time} => {}
        Command::Rounds {amount} => {}
    }
}
