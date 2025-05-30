# Rust Pomodoro 🦀🍅

A simple and configurable Pomodoro timer built in Rust. Use it to stay focused, 
manage breaks, and track your work sessions—all from the command line.

## Features

* Start a Pomodoro session with custom focus/break times and rounds
* Persist your preferred settings between runs
* Update settings independently (focus time, break time, rounds)
* Reset all settings to default

## Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/pomodoro-cli.git
cd pomodoro-cli
cargo build --release
```

Then you can run the binary:

```bash
./target/release/pomodoro
```

## Usage

```bash
pomodoro <COMMAND>
```

### Commands

#### `start`

Start a Pomodoro session. You can optionally override settings with flags.

```bash
pomodoro start [--focus <minutes>] [--break <minutes>] [--rounds <count>]
```

* `--focus`: Duration of the focus session in minutes
* `--break`: Duration of the break session in minutes
* `--rounds`: Number of focus/break rounds

#### `focus`

Update the default focus session time.

```bash
pomodoro focus <minutes>
```

#### `break`

Update the default break session time.

```bash
pomodoro break <minutes>
```

#### `rounds`

Update the default number of rounds.

```bash
pomodoro rounds <count>
```

#### `reset`

Reset all saved settings to defaults.

```bash
pomodoro reset
```

## Configuration

User preferences can be saved between runs. The configuration is read from and written to a file automatically.

If you provide any values via command-line flags during a `start`, they will override the saved config temporarily for that session.

## Example

```bash
# Start a 25/5 pomodoro session with 4 rounds
pomodoro start --focus 25 --break 5 --rounds 4

# Set focus time to 30 minutes
pomodoro focus 30

# Reset all saved preferences
pomodoro reset
```

## License

MIT

---

Built with ❤️ in Rust.
