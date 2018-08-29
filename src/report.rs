use std::collections::HashMap;
use log_reader::LogEvent;
use serde_json;
use itertools::any;
use serde;

fn print_to_console(log_events: &Vec<LogEvent>) {
    for log_event in log_events {
        println!("{} -- {} -- {}", &log_event.date, &log_event.context_identifier, &log_event.name);
        for payload_item in &log_event.payload {
            print!(" {} ", payload_item);
        }
        println!("\n");
    }
}

fn print_validation_results(check_list_results: &HashMap<String, bool>) {
    for (config_step_name, value) in check_list_results.iter() {
        println!("Found: {} -- Config Step: {}", value, config_step_name);
    }
    if any(check_list_results.values(), |value| value == &false) {
        println!("Validation failed: Items are missing");
    } else {
        println!("Validation passed. No missing items");
    }
}

pub fn print_json<T: serde::Serialize>(payload: T) {
    let json = serde_json::to_string_pretty(&payload).expect("Failed to serialize to JSON");
    println!("{}", json);
}

pub fn print_log_event(log_events: &Vec<LogEvent>, wants_json: bool) {
    if wants_json {
        print_json(log_events);
    } else {
        print_to_console(log_events);
    }
}

pub fn print_workflow_results_for_single_checklist(check_list_results: HashMap<String, bool>, wants_json: bool) {
    if wants_json {
        print_json(check_list_results);
    } else {
        print_validation_results(&check_list_results);
    }
}

pub fn print_workflow_results_for_all_checklists(check_lists: HashMap<String, HashMap<String, bool>>, wants_json: bool) {
    if wants_json {
        print_json(check_lists);
    } else {
        for (context_identifier, check_list) in check_lists.iter() {
            println!("ID: {}", context_identifier);
            print_validation_results(check_list);
        }
    }
}