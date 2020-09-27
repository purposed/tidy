mod tidy;

use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

use anyhow::Result;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use rood::cli::OutputManager;

use tidy::Engine;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn cli_run(matches: &ArgMatches) -> Result<()> {
    let verbose = matches.is_present("verbose");
    let config_path = matches.value_of("config").unwrap(); // Mandatory argument.

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let raw_data = fs::read_to_string(config_path)?;
    let mut engine = Engine::new(&raw_data, verbose)?;
    engine.start()?;

    while running.load(Ordering::SeqCst) {
        thread::sleep(time::Duration::from_millis(1000));
    }
    engine.stop()
}

fn cli_dispatch(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("run", Some(m)) => cli_run(m),
        _ => Ok(()),
    }
}

fn main() {
    let app = App::new("tidy")
        .version(VERSION)
        .author("Purposed")
        .about("Rapid filesystem organizer")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Starts the Tidy watcher")
                .arg(
                    Arg::with_name("config")
                        .long("cfg")
                        .short("c")
                        .help("Path to the config file to use")
                        .required(false)
                        .default_value("config.json"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short("v")
                        .help("Whether to use verbose output")
                        .required(false),
                ),
        );

    match cli_dispatch(app.get_matches()) {
        Ok(_) => {}
        Err(e) => OutputManager::new(false).error(&format!("{}", e)),
    }
}
