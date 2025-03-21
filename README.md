# Pomodoro Timer CLI

A simple command-line Pomodoro timer written in Rust that plays a sound when a timer completes.

## Features

- Configurable work duration, break duration, and long break duration
- Customizable number of cycles before a long break
- Built-in default sound or custom sound file option
- Clear countdown display in the terminal
- Pause between cycles to allow user to prepare

## Installation

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs))
- Audio libraries dependencies for rodio (varies by platform)

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/pomodoro-timer.git
cd pomodoro-timer

# Build the project
cargo build --release

# The executable will be in target/release/pomodoro-timer
```

## Usage

```bash
# Run with default settings (25min work, 5min break, 15min long break, 4 cycles)
./pomodoro-timer

# Custom work and break durations
./pomodoro-timer --work 50 --break 10

# Custom sound file (must be in WAV format)
./pomodoro-timer --sound-file /path/to/your/sound.wav

# Full options
./pomodoro-timer --work 30 --break 5 --long-break 20 --cycles 3 --sound-file alarm.wav
```

### Command-line options

```
USAGE:
    pomodoro-timer [OPTIONS]

OPTIONS:
    -w, --work <MINUTES>            Work duration in minutes [default: 25]
    -b, --break <MINUTES>           Break duration in minutes [default: 5]
    -l, --long-break <MINUTES>      Long break duration in minutes [default: 15]
    -c, --cycles <COUNT>            Number of work/break cycles before a long break [default: 4]
    -s, --sound-file <FILE>         Custom sound file to play (WAV format)
    -h, --help                      Prints help information
    -V, --version                   Prints version information
```

## Audio

The timer includes a built-in beep sound that will play when timers complete, with no external files needed. If you prefer a different sound, you can specify your own WAV file using the `--sound-file` option.

## License

This project is licensed under the MIT License - see the LICENSE file for details.