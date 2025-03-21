use clap::{App, Arg};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::thread::sleep;
use std::time::{Duration, Instant};

// Built-in alert sound as bytes, so we don't need an external file
const ALERT_SOUND: &[u8] = include_bytes!("alert.wav");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Pomodoro Timer")
        .version("1.0")
        .author("Your Name")
        .about("A simple Pomodoro timer CLI")
        .arg(
            Arg::with_name("work")
                .short("w")
                .long("work")
                .value_name("MINUTES")
                .help("Work duration in minutes")
                .default_value("25"),
        )
        .arg(
            Arg::with_name("break")
                .short("b")
                .long("break")
                .value_name("MINUTES")
                .help("Break duration in minutes")
                .default_value("5"),
        )
        .arg(
            Arg::with_name("long-break")
                .short("l")
                .long("long-break")
                .value_name("MINUTES")
                .help("Long break duration in minutes")
                .default_value("15"),
        )
        .arg(
            Arg::with_name("cycles")
                .short("c")
                .long("cycles")
                .value_name("COUNT")
                .help("Number of work/break cycles before a long break")
                .default_value("4"),
        )
        .arg(
            Arg::with_name("sound-file")
                .short("s")
                .long("sound-file")
                .value_name("FILE")
                .help("Custom sound file to play (WAV format)")
                .takes_value(true),
        )
        .get_matches();

    let work_duration = matches
        .value_of("work")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(25);
    let break_duration = matches
        .value_of("break")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(5);
    let long_break_duration = matches
        .value_of("long-break")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(15);
    let cycles = matches
        .value_of("cycles")
        .unwrap()
        .parse::<u32>()
        .unwrap_or(4);
    let custom_sound_file = matches.value_of("sound-file");

    run_pomodoro(
        work_duration,
        break_duration,
        long_break_duration,
        cycles,
        custom_sound_file,
    )?;

    Ok(())
}

fn run_pomodoro(
    work_duration: u64,
    break_duration: u64,
    long_break_duration: u64,
    cycles: u32,
    custom_sound_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cycle_count = 1;

    loop {
        // Work session
        run_timer("Work", work_duration * 60)?;
        play_sound(custom_sound_file)?;
        println!("\nWork session completed! Time for a break.");

        // Determine if it's time for a long break
        if cycle_count >= cycles {
            run_timer("Long Break", long_break_duration * 60)?;
            play_sound(custom_sound_file)?;
            println!("\nLong break completed! Ready for a new set of cycles.");
            cycle_count = 1;
        } else {
            run_timer("Break", break_duration * 60)?;
            play_sound(custom_sound_file)?;
            println!("\nBreak completed! Back to work.");
            cycle_count += 1;
        }

        println!("\nPress Enter to continue or Ctrl+C to exit...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    }
}

fn run_timer(label: &str, seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let total_duration = Duration::from_secs(seconds);

    while start_time.elapsed() < total_duration {
        let elapsed = start_time.elapsed();
        let remaining = total_duration.checked_sub(elapsed).unwrap_or_default();
        
        let mins = remaining.as_secs() / 60;
        let secs = remaining.as_secs() % 60;
        
        print!("\r{} time remaining: {:02}:{:02}", label, mins, secs);
        std::io::Write::flush(&mut std::io::stdout())?;
        
        // Update less frequently to reduce CPU usage
        sleep(Duration::from_millis(500));
    }
    
    Ok(())
}

fn play_sound(custom_sound_file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Use either the custom sound file or the default built-in sound
    let source = if let Some(file_path) = custom_sound_file {
        let file = BufReader::new(File::open(file_path)?);
        Decoder::new(file)?
    } else {
        // Use the built-in alert sound
        let cursor = Cursor::new(ALERT_SOUND);
        Decoder::new(cursor)?
    };

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}