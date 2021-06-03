use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;

use chrono::{prelude::*, Duration};
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};

/// Default timer name.
const NAME_DEFAULT: &str = "main";

/// Timers path.
const TIMERS_PATH: &str = "peperone/timers.toml";

/// Main, start the program.
fn main() {
    // Match CLI arguments
    let matches = App::new("peperone")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("new")
                .alias("n")
                .alias("create")
                .alias("start")
                .alias("s")
                .about("Start new timer")
                .arg(
                    Arg::new("NAME")
                        .about("Timer name")
                        .default_value(NAME_DEFAULT),
                ),
        )
        .subcommand(
            App::new("remove")
                .alias("rm")
                .alias("r")
                .alias("del")
                .about("Remove timer")
                .arg(
                    Arg::new("NAME")
                        .about("Timer name")
                        .default_value(NAME_DEFAULT),
                ),
        )
        .subcommand(App::new("list").alias("ls").alias("l").about("List timers"))
        .subcommand(
            App::new("show")
                .alias("cat")
                .alias("info")
                .alias("view")
                .alias("status")
                .about("Show timer")
                .arg(
                    Arg::new("NAME")
                        .about("Timer name")
                        .default_value(NAME_DEFAULT),
                )
                .arg(
                    Arg::new("quiet")
                        .long("quiet")
                        .short('q')
                        .about("Quiet output"),
                ),
        )
        .get_matches();

    // Load timers
    let mut timers = Timers::load();

    // Handle specific command
    if let Some(matcher) = matches.subcommand_matches("new") {
        new(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("remove") {
        remove(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("list") {
        list(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("show") {
        show(matcher, &mut timers);
    } else {
        unreachable!()
    }
}

/// A list of timers.
#[derive(Serialize, Deserialize, Debug, Default)]
struct Timers {
    pub timers: HashMap<String, Timer>,
}

impl Timers {
    /// Load timers from file.
    fn load() -> Timers {
        // If no timers file, load empty
        if !timers_path().is_file() {
            return Timers::default();
        }

        // Load timers file
        let data = fs::read(timers_path()).expect("failed to read timers file");
        toml::from_slice(&data).expect("failed to deserialize timers")
    }

    /// Save timers to file.
    fn save(&self) {
        // Ensure parent directory exists
        let path = timers_path();
        let parent = path.parent().expect("failed to determine parent path");
        fs::create_dir_all(parent).expect("failed to create parent directories for tiemrs file");

        // Write file
        let data = toml::to_string(self).expect("failed to serialize timers");
        fs::write(timers_path(), data).expect("failed to write timers file");

        // TODO: remove file and parent dir if timers are empty
    }
}

impl Drop for Timers {
    fn drop(&mut self) {
        self.save();
    }
}

/// A timer.
#[derive(Serialize, Deserialize, Debug)]
struct Timer {
    start: DateTime<Utc>,
}

impl Timer {
    /// Create and start new timer.
    fn new() -> Timer {
        Timer { start: Utc::now() }
    }

    /// Get elapsed time.
    fn elapsed(&self) -> Duration {
        return Utc::now() - self.start;
    }
}

/// Get path to timers file.
fn timers_path() -> PathBuf {
    dirs::cache_dir()
        .expect("cache dir cannot be found")
        .join(TIMERS_PATH)
        .into()
}

/// Start a new timer.
fn new(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    timers.timers.insert(name.into(), Timer::new());
}

/// Remove a timer.
fn remove(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    if timers.timers.remove(name).is_none() {
        eprintln!("error: no timer named '{}'", name);
        process::exit(1);
    }
}

/// List all timers.
fn list(_matcher: &ArgMatches, timers: &mut Timers) {
    for name in timers.timers.keys() {
        println!("{}", name);
    }
}

/// Show a timer.
fn show(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    let quiet = matcher.is_present("quiet");

    // Get timer and elapsed time
    let timer = match timers.timers.get(name) {
        Some(timer) => timer,
        None => {
            if !quiet {
                eprintln!("error: no timer named '{}'", name);
            }
            process::exit(1);
        }
    };
    let elapsed = timer.elapsed();

    // Print to console
    let mut format = format!(
        "{:02}:{:02}",
        elapsed.num_minutes() & 60,
        elapsed.num_seconds() % 60
    );
    if elapsed.num_hours() > 0 {
        format = format!("{}:{}", format, elapsed.num_hours());
    }
    println!("{}", format);
}
