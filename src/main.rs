extern crate clap;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate regex;
extern crate itertools;

mod config;
mod log_reader;
mod aggregator;
mod console;
mod validator;

use std::io::Read;
use std::fs::File;
use std::error::Error;

use clap::{Arg, App, SubCommand};
use itertools::{Itertools, any};

fn read_log_file(path: &str) -> Result<String, Box<Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn validate_workflow(log_events: &Vec<log_reader::LogEvent>, config_file: &config::ConfigFile) {
    let check_list_results = validator::validate_single(log_events, &config_file);
    for (config_step_name, value) in check_list_results.iter() {
        println!("Found: {} -- Config Step: {}", value, config_step_name);
    }

    if any(check_list_results.values(), |value| value == &false) {
        println!("Validation failed: Items are missing");
    } else {
        println!("Validation passed. No missing items");
    }
}

fn main() {
    let matches = App::new("Log Analyzer")
                          .version("1.0")
                          .author("Jan Schulte <hello@unexpected-co.de>")
                          .about("Analyzes your log files")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("filter")
                                .short("f")
                                .long("filter")
                                .value_name("FILTER")
                                .help("Filter aggregated logs by a certain value")
                                .takes_value(true))
                          .arg(Arg::with_name("context-identifier-only")
                                    .long("context-ids-only")
                                    .help("Prints out only the context identifier for each collection of events")
                                    .conflicts_with("validate-workflow")
                                    .takes_value(false))
                          .arg(Arg::with_name("validate-workflow")
                                    .long("validate-workflow")
                                    .help("validates if a log event complies to all steps from configuration file")
                                    .takes_value(false)
                                    .conflicts_with("context-identifiers-only"))
                          .subcommand(SubCommand::with_name("config")
                                      .arg(Arg::with_name("validate")
                                          .short("v")
                                          .value_name("FILE")
                                          .takes_value(true)
                                          .required(true)
                                          .help("Validates your config file")))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("config") {
        if matches.is_present("validate") {
            println!("Validating config file...");
            let filename = matches.value_of("validate").unwrap();
            match config::read_config_from_file(filename) {
                Ok(_) => println!("config valid"),
                Err(err) => println!("{:?}", err)
            }
        }
        return;
    }

    let config_filename = matches.value_of("config").unwrap_or("config.json");
    println!("Value for config: {}", config_filename);
    let input_file_path = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", input_file_path);

    let file = match read_log_file(input_file_path) {
        Ok(f) => f,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let config_file = match config::read_config_from_file(config_filename) {
        Ok(c) => c,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let log_events = log_reader::extract(&config_file, file);
    let aggregated = aggregator::aggregate(log_events);

    if matches.is_present("context-identifier-only") {
        for key in aggregated.keys().unique().collect::<Vec<_>>() {
            println!("{}", key);
        }
    }
    else {
        if let Some(filter_argument) = matches.value_of("filter") {
            if let Some(log_events) = aggregated.get(filter_argument) {
                if matches.is_present("validate-workflow") {
                    validate_workflow(log_events, &config_file);
                } else {
                    console::print_log_event(log_events);
                }
            }
        } else {
            for (_context_identifier, log_events)  in aggregated.iter() {
                console::print_log_event(log_events);
            }
        }
    }
}
