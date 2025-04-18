use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Rust Pomodoro 🦀🍅")]
#[command(about = "A CLI Pomodoro timer written in Rust", long_about = None)]
pub struct Cli {
    /// Focus time in minutes
    #[arg(long, default_value_t = 25)]
    pub focus: u32,

    /// Break time in minutes
    #[arg(long, default_value_t = 5)]
    pub break_: u32,

    /// Number of rounds for the pomodoro session
    #[arg(long, default_value_t = 3)]
    pub rounds: u32,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the Pomodoro timer
    Start {
        /// Specify the focus interval for this session
        #[arg(long)]
        focus: Option<u32>,

        /// Specify the break interval for this session
        #[arg(long)]
        break_: Option<u32>,

        /// Specify the rounds for this session
        #[arg(long)]
        rounds: Option<u32>,
    }
}
