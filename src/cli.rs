use clap::{Parser, Subcommand};

// TODO: Add functionality to save configuration.
#[derive(Parser, Debug)]
#[command(name = "Rust Pomodoro 🦀🍅")]
#[command(about = "A CLI Pomodoro timer written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the Pomodoro timer
    Start {
        /// Specify the focus interval for this session
        #[arg(long)]
        focus: Option<u64>,

        /// Specify the break interval for this session
        #[arg(long)]
        break_: Option<u64>,

        /// Specify the rounds for this session
        #[arg(long)]
        rounds: Option<u64>,
    },
    /// Focus time in minutes
    Focus {
        #[arg(default_value_t = 25)]
        time: u64,
    },

    /// Break time in minutes
    Break {
        #[arg(default_value_t = 5)]
        time: u64,
    },

    /// Number of rounds for the pomodoro session
    Rounds {
        #[arg(default_value_t = 3)]
        amount: u64,
    },
}
