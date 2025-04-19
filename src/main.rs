mod cli;
mod config;

use clap::Parser;
use cli::{Cli};

fn main() {
    println!("Hello, world!");
    let _ = Cli::parse();
}
