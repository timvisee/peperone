use clap::{App, Arg, ArgMatches};

/// Default timer name.
const NAME_DEFAULT: &str = "main";

fn main() {
    // Match CLI arguments
    let matches = App::new("peperone")
        .subcommand(
            App::new("start")
                .alias("new")
                .alias("s")
                .about("Start timer")
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
                // TODO: must be a flag
                .arg(Arg::new("quiet").visible_alias("q").about("Quiet output")),
        )
        .get_matches();

    // Handle specific command
    if let Some(matcher) = matches.subcommand_matches("start") {
        start(matcher);
    } else if let Some(matcher) = matches.subcommand_matches("remove") {
        remove(matcher);
    } else if let Some(matcher) = matches.subcommand_matches("list") {
        list(matcher);
    } else if let Some(matcher) = matches.subcommand_matches("show") {
        show(matcher);
    } else {
        // TODO: nothing selected, shouldn't happen
    }
}

fn start(matcher: &ArgMatches) {
    eprintln!("START");
}

fn remove(matcher: &ArgMatches) {
    eprintln!("REMOVE");
}

fn list(matcher: &ArgMatches) {
    eprintln!("LIST");
}

fn show(matcher: &ArgMatches) {
    eprintln!("SHOW");
}
