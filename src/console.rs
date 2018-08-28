use log_reader::LogEvent;

pub fn print_log_event(log_events: &Vec<LogEvent>) {
    for log_event in log_events {
        println!("{} -- {} -- {}", &log_event.date, &log_event.context_identifier, &log_event.name);
        for payload_item in &log_event.payload {
            print!(" {} ", payload_item);
        }
        println!("\n");
    }
}