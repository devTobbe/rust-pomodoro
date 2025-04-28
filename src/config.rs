use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::from_str;

// Configuration struct to enable customization
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    focus: Option<u64>,
    break_: Option<u64>,
    rounds: Option<u64>,
}

// Implemented functinos for the config struct
impl Config {
    // Create a new config object
    pub fn new(focus: Option<u64>, break_: Option<u64>, rounds: Option<u64>) -> Self {
        Self {
            focus,
            break_,
            rounds,
        }
    }
}

// Default configuration that is used if nothing else is specified.
const DEFAULT_CONFIG: Config = Config {
    focus: Some(25),
    break_: Some(5),
    rounds: Some(4),
};

// Read configuration file, if nonexistent, return defaults instead.
pub fn readfile() -> Config {
    
    // Check if there is a configuration file, if not, return default config
    if !Path::new("conf.json").exists() {
        return DEFAULT_CONFIG;
    }

    // Read configuation file
    let file = std::fs::read_to_string("conf.json").unwrap();
    let file_conf: Config = from_str(&file).unwrap();

    return file_conf;
}

// TODO: Improve error handling
pub fn writefile(conf : &Config) {

    // Check if there is a configuration file, if not, create one
    if !Path::new("conf.json").exists() {
        let _ = File::create("conf.json");
    }

    let json = serde_json::to_string_pretty(&conf).unwrap();
    let _ = std::fs::write("conf.json", json);
}
