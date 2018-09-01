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

use std::collections::HashMap;

use clap::{Arg, App, SubCommand};
use itertools::Itertools;

fn validate_workflow_for_file(aggregated_logs: aggregator::AggregatedLogs, config_file: &config::ConfigFile, wants_json: bool) {
    let mut validation_results : HashMap<String, HashMap<String, bool>> = HashMap::new();
    for (context_identifier, log_events) in aggregated_logs.events_by_context_id {
        validation_results.insert(context_identifier, validator::validate_single(&log_events, &config_file));
    }
    report::print_workflow_results_for_all_checklists(validation_results, wants_json);
}

fn validate_workflow_for_single_context_id(log_events: &Vec<log_reader::LogEvent>, config_file: &config::ConfigFile, wants_json: bool) {
    let check_list_results = validator::validate_single(log_events, &config_file);
    report::print_workflow_results_for_single_checklist(check_list_results, wants_json);
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
                                    .help("Validates if a log event complies to all steps from configuration file")
                                    .takes_value(false)
                                    .conflicts_with("context-identifiers-only"))
                          .arg(Arg::with_name("json-report")
                                    .long("json-report")
                                    .help("Generates a report in the JSON format")
                                    .takes_value(false))
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

    let wants_json = matches.is_present("json-report");

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
        if wants_json {
            report::print_json(ids);
        } else {
            for key in ids {
                println!("{}", key);
            }
        }
    }
    else {
        if let Some(filter_argument) = matches.value_of("filter") {
            if let Some(log_events) = aggregated.events_by_context_id.get(filter_argument) {
                if matches.is_present("validate-workflow") {
                    validate_workflow_for_single_context_id(log_events, &config_file, wants_json);
                } else {
                    report::print_log_event(log_events, wants_json);
                }
            }
        } else {
            if matches.is_present("validate-workflow") {
                validate_workflow_for_file(aggregated, &config_file, wants_json);
                println!("\n");
            } else {
                for (_context_identifier, log_events)  in aggregated.events_by_context_id.iter() {
                    report::print_log_event(log_events, wants_json);
                }
            }
        }
    }
}
