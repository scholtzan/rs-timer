use std::time::{Duration, Instant};
use std::thread;
use std::io::{self, Write};
use clap::{Arg, App};

#[macro_use]
extern crate clap;


fn main() {
    let matches = App::new("rs-timer")
                      .version("0.1.0")
                      .author("Anna Scholtz <anna@scholtzan.net>")
                      .about("Simple timer implemented with Rust")
                      .arg(Arg::with_name("seconds")
                           .short("s")
                           .long("seconds")
                           .help("Sets the seconds of the timer")
                           .takes_value(true))
                      .arg(Arg::with_name("minutes")
                           .short("m")
                           .long("minutes")
                           .help("Sets the minutes of the timer")
                           .takes_value(true))
                      .arg(Arg::with_name("hours")
                           .short("h")
                           .long("hours")
                           .help("Sets the hours of the timer")
                           .takes_value(true))
                      .get_matches();

    // parse the command line arguments
    let seconds = value_t!(matches, "seconds", u64).unwrap_or(0);
    let minutes = value_t!(matches, "minutes", u64).unwrap_or(0);
    let hours = value_t!(matches, "hours", u64).unwrap_or(0);

    let total_seconds = seconds + minutes * 60 + hours * 3600;

    start_timer(total_seconds);
}


/// Starts a timer that runs for the number of seconds given.
///
/// The timer outputs the remaining time which is updated every second.
fn start_timer(total_seconds: u64) {
    let maximum_duration = Duration::from_secs(total_seconds);
    let start_time = Instant::now();

    while maximum_duration >= start_time.elapsed() {
        print_formatted_time((maximum_duration - start_time.elapsed()).as_secs());

        thread::sleep(Duration::from_secs(1));

        clear_command_line();
    }
}


/// Prints a passed number of seconds as time string with format hh:mm:ss.
fn print_formatted_time(remaining_seconds: u64) {
    let hours = remaining_seconds / 3600;
    let minutes = (remaining_seconds - (hours * 3600)) / 60;
    let seconds = remaining_seconds - (hours * 3600) - (minutes * 60);

    print!("{}:{}:{}", format_time(hours), format_time(minutes), format_time(seconds));
    io::stdout().flush().unwrap();
}


/// Converts passed numbers to strings and adds leading zeros if they are smaller 10.
fn format_time(time: u64) -> String {
    match time {
        0 ... 9 => "0".to_string() + &time.to_string(),
        _ => time.to_string(),
    }
}


/// Removes all output from the command line.
fn clear_command_line() {
    print!("\r");
    io::stdout().flush().unwrap();
}
