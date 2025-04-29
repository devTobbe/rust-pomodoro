# Rust Pomodoro 🦀🍅

A simple and customizable Pomodoro timer written in Rust.

## Features

- 📦 Lightweight CLI interface

- ⏱ Customizable focus and break durations

- 🔁 Adjustable number of Pomodoro rounds

- 🧠 Great for boosting productivity using the Pomodoro Technique!

## Install

TODO: Instructions for installing the CLI tool.

## Usage

Once the binary is built and available on your system, you can use the following CLI commands:

### Start a Pomodoro session

rust-pomodoro start

This will start a session with the default values:
• Focus: 25 minutes
• Break: 5 minutes
• Rounds: 3

### Override defaults for a single session

rust-pomodoro start --focus 30 --break 10 --rounds 4

This command runs a Pomodoro session with:
• 30-minute focus intervals
• 10-minute breaks
• 4 total rounds

### Set global defaults and run

rust-pomodoro --focus 40 --break 10 --rounds 5 start

This sets new default values (only for this run of the program) and starts the timer.

## Options

Global flags (apply as defaults to any command):
• --focus <minutes> — Set default focus time (default: 25)
• --break <minutes> — Set default break time (default: 5)
• --rounds <number> — Set default number of Pomodoro rounds (default: 3)

Subcommand: start
You can use start with optional overrides:
• --focus <minutes> — Override focus time for this session
• --break <minutes> — Override break time for this session
• --rounds <number> — Override round count for this session

## TODO

- 💾 Save configuration to a file

- v🔔 Add notification support

- 🎵 Optional sounds for focus/break transitions

- 🪟 Optional GUI frontend

## License
