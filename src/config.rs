use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::from_str;

// Configuration struct to enable customization
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub focus: u64,
    pub break_: u64,
    pub rounds: u64,
}

// Implemented functinos for the config struct
impl Config {
    // Create a new config object
    pub fn new(focus: u64, break_: u64, rounds: u64) -> Self {
        Self {
            focus,
            break_,
            rounds,
        }
    }
}

// Default configuration that is used if nothing else is specified.
const DEFAULT_CONFIG: Config = Config {
    focus: 25,
    break_: 5,
    rounds: 4,
};

// Read configuration file, if nonexistent, return defaults instead.
pub fn readfile() -> Config {
    // Check if there is a configuration file, if not, return default config
    if !Path::new("conf.json").exists() {
        return DEFAULT_CONFIG;
    }

    // Read configuation file
    match std::fs::read_to_string("conf.json") {
        Ok(contents) => match from_str(&contents) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("Warning: Failed to parse conf.json, using default config.");
                return DEFAULT_CONFIG;
            }
        },
        Err(_) => {
            eprintln!("Warning: Failed to read conf.json, using default config.");
            return DEFAULT_CONFIG;
        }
    }
}

// TODO: Improve error handling
pub fn writefile(conf: &Config) {
    // Check if there is a configuration file, if not, create one
    if !Path::new("conf.json").exists() {
        let _ = File::create("conf.json");
    }

    let json = serde_json::to_string_pretty(&conf).unwrap();
    let _ = std::fs::write("conf.json", json);
}

// Updates the specified attribute in the configuration file
pub fn update_saved_attribute(attr: String, val: u64) {
    // Read the exisitng file
    let mut conf = readfile();
    // Check which attribute was selected
    match attr.as_str() {
        "focus" => {
            conf.focus = val;
        }
        "break" => {
            conf.break_ = val;
        }
        "rounds" => {
            conf.rounds = val;
        }
        _ => {
            print!("Error, add error later");
            return;
        }
    }

    // Write to file
    writefile(&conf);
}
