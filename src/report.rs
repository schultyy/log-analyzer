use log_reader::LogEvent;
use serde_json;

fn print_to_console(log_events: &Vec<LogEvent>) {
    for log_event in log_events {
        println!("{} -- {} -- {}", &log_event.date, &log_event.context_identifier, &log_event.name);
        for payload_item in &log_event.payload {
            print!(" {} ", payload_item);
        }
        println!("\n");
    }
}

fn print_json(log_events: &Vec<LogEvent>) {
    let json = serde_json::to_string_pretty(&log_events).expect("Failed to serialize Log Events to JSON");
    println!("{}", json);
}

pub fn print_log_event(log_events: &Vec<LogEvent>, wants_json: bool) {
    if wants_json {
        print_json(log_events);
    } else {
        print_to_console(log_events);
    }
}
