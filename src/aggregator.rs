use log_reader::LogEvent;
use std::collections::HashMap;

pub fn aggregate(log_events: Vec<LogEvent>) -> HashMap<String, Vec<LogEvent>> {
    let mut hashmap : HashMap<String, Vec<LogEvent>> = HashMap::new();

    for log_event in log_events {
        let context_identifier = log_event.context_identifier.clone();
        if hashmap.contains_key(&context_identifier) {
            let list = hashmap.get_mut(&context_identifier).unwrap();
            list.push(log_event);
        } else {
            let log_events_by_key = vec!(log_event);
            hashmap.insert(context_identifier, log_events_by_key);
        }
    }

    hashmap
}