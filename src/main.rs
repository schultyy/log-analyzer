extern crate clap;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate regex;
extern crate itertools;

mod config;
mod log_reader;
mod aggregator;
mod report;
mod validator;

use clap::{Arg, App, SubCommand};
use itertools::Itertools;

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
                                    .help("Validates if a log event complies to all steps from configuration file")
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

    let config_file = match config::read_config_from_file(config_filename) {
        Ok(c) => c,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let log_results = match log_reader::extract(&config_file, input_file_path) {
        Ok(results) => results,
        Err(err) => {
            println!("ERR: {:?}", err);
            return;
        }
    };
    let aggregated = aggregator::aggregate(log_results);

    if matches.is_present("context-identifier-only") {
        let ids = aggregated.events_by_context_id.keys().unique().collect::<Vec<_>>();
        report::print_json(ids);
    }
    else {
        if let Some(filter_argument) = matches.value_of("filter") {
            if let Some(log_events) = aggregated.events_by_context_id.get(filter_argument) {
                if matches.is_present("validate-workflow") {
                    let validation_results = validator::validate_workflow_for_single_context_id(aggregated.log_filename, log_events, &config_file);
                    report::print_json(validation_results);
                } else {
                    report::print_json(log_events);
                }
            }
        } else {
            if matches.is_present("validate-workflow") {
                let validation_results = validator::validate_workflow_for_file(aggregated, &config_file);
                report::print_json(validation_results);
                println!("\n");
            } else {
                report::print_json(aggregated);
            }
        }
    }
}
