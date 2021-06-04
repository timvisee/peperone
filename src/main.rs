use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::mpsc::channel;

use chrono::{prelude::*, Duration};
use clap::{App, AppSettings, Arg, ArgMatches};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
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
                .about("Create and start timer")
                .arg(
                    Arg::new("NAME")
                        .about("Timer name")
                        .default_value(NAME_DEFAULT),
                ),
        )
        .subcommand(
            App::new("start")
                .alias("s")
                .about("Start existing timer")
                .arg(
                    Arg::new("NAME")
                        .about("Timer name")
                        .default_value(NAME_DEFAULT),
                ),
        )
        .subcommand(
            App::new("stop").alias("pause").about("Stop timer").arg(
                Arg::new("NAME")
                    .about("Timer name")
                    .default_value(NAME_DEFAULT),
            ),
        )
        .subcommand(
            App::new("toggle")
                .alias("startstop")
                .about("Toggle timer (start/stop)")
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
        .subcommand(
            App::new("tail")
                .about("Tail a timer")
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
    } else if let Some(matcher) = matches.subcommand_matches("start") {
        start(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("stop") {
        stop(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("toggle") {
        toggle(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("remove") {
        remove(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("list") {
        list(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("show") {
        show(matcher, &mut timers);
    } else if let Some(matcher) = matches.subcommand_matches("tail") {
        tail(matcher, &mut timers);
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

/// A timer.
#[derive(Serialize, Deserialize, Debug, Default)]
struct Timer {
    /// If active, last time we started counting at.
    #[serde(default)]
    start: Option<DateTime<Utc>>,

    /// Additional elapsed time.
    #[serde(default)]
    offset: std::time::Duration,
}

impl Timer {
    /// Create and start new timer.
    pub fn new() -> Timer {
        let mut timer = Timer::default();
        timer.start();
        timer
    }

    /// Whether the timer is running.
    pub fn running(&self) -> bool {
        self.start.is_some()
    }

    /// (Re)start the timer.
    pub fn start(&mut self) {
        if let Some(start) = self.start.replace(Utc::now()) {
            self.offset += (Utc::now() - start).to_std().unwrap();
        }
    }

    /// Stop/pause the timer.
    pub fn stop(&mut self) {
        if let Some(start) = self.start.take() {
            self.offset += (Utc::now() - start).to_std().unwrap();
        }
    }

    /// Elapsed time.
    pub fn elapsed(&self) -> Duration {
        let mut elapsed = Duration::from_std(self.offset).unwrap();
        if let Some(start) = self.start {
            elapsed = elapsed + (Utc::now() - start);
        }
        elapsed
    }

    /// Format elapsed time.
    pub fn format_elapsed(&self) -> String {
        let elapsed = self.elapsed();

        // Print to console
        let mut format = format!(
            "{}:{:02}",
            elapsed.num_minutes() % 60,
            elapsed.num_seconds() % 60,
        );
        if elapsed.num_hours() > 0 {
            format = format!("{}:{}", elapsed.num_hours(), format);
        }

        format
    }
}

/// Get path to timers file.
fn timers_path() -> PathBuf {
    dirs::cache_dir()
        .expect("cache dir cannot be found")
        .join(TIMERS_PATH)
        .into()
}

/// Create and start new timer.
fn new(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    timers.timers.insert(name.into(), Timer::new());
    timers.save();
}

/// Start existing timer.
fn start(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    match timers.timers.get_mut(name) {
        Some(timer) => timer.start(),
        None => {
            eprintln!("error: no timer named '{}'", name);
            process::exit(1);
        }
    }
    timers.save();
}

/// Stop/pause existing timer.
fn stop(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    match timers.timers.get_mut(name) {
        Some(timer) => timer.stop(),
        None => {
            eprintln!("error: no timer named '{}'", name);
            process::exit(1);
        }
    }
    timers.save();
}

/// Toggle existing timer.
fn toggle(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    match timers.timers.get_mut(name) {
        Some(timer) if timer.running() => timer.stop(),
        Some(timer) => timer.start(),
        None => {
            eprintln!("error: no timer named '{}'", name);
            process::exit(1);
        }
    }
    timers.save();
}

/// Remove a timer.
fn remove(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    if timers.timers.remove(name).is_none() {
        eprintln!("error: no timer named '{}'", name);
        process::exit(1);
    }
    timers.save();
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

    let timer = match timers.timers.get(name) {
        Some(timer) => timer,
        None => {
            if !quiet {
                eprintln!("error: no timer named '{}'", name);
            }
            process::exit(1);
        }
    };
    println!("{}", timer.format_elapsed());
}

/// Tail a timer.
fn tail(matcher: &ArgMatches, timers: &mut Timers) {
    let name = matcher.value_of("NAME").unwrap();
    let quiet = matcher.is_present("quiet");

    // Load timer
    let mut timer = match timers.timers.get(name) {
        Some(timer) => timer,
        None => {
            if !quiet {
                eprintln!("error: no timer named '{}'", name);
            }
            process::exit(1);
        }
    };

    // Create timer file watcher
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::seconds(1).to_std().unwrap()).unwrap();
    watcher
        .watch(timers_path(), RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        // Process all file events, determine whether to recheck
        let recheck = rx.try_iter().fold(false, |recheck, e| {
            recheck
                || match e {
                    DebouncedEvent::NoticeWrite(_) => false,
                    DebouncedEvent::NoticeRemove(_) => false,
                    DebouncedEvent::Create(_) => true,
                    DebouncedEvent::Write(_) => true,
                    DebouncedEvent::Chmod(_) => false,
                    DebouncedEvent::Remove(_) => true,
                    DebouncedEvent::Rename(_, _) => true,
                    DebouncedEvent::Rescan => true,
                    DebouncedEvent::Error(_, _) => true,
                }
        });

        // Recheck timer, make sure it's still active
        if recheck {
            *timers = Timers::load();
            timer = match timers.timers.get(name) {
                Some(timer) => timer,
                None => process::exit(0),
            };
        }

        // Print time if running
        if timer.running() {
            println!("{}", timer.format_elapsed());
        }

        // Wait for next tick
        if timer.running() {
            std::thread::sleep(std::time::Duration::from_millis(
                (1000 - timer.elapsed().num_milliseconds() % 1000) as u64,
            ));
        } else {
            std::time::Duration::from_millis(100);
        }
    }
}
