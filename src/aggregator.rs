use log_reader::{LogResults, LogEvent};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct AggregatedLogs {
    pub log_filename: String,
    pub events_by_context_id: HashMap<String, Vec<LogEvent>>
}

pub fn aggregate(log_results: LogResults) -> AggregatedLogs {
    let mut events_by_context_id : HashMap<String, Vec<LogEvent>> = HashMap::new();

    for log_event in log_results.events {
        let context_identifier = log_event.context_identifier.clone();
        if events_by_context_id.contains_key(&context_identifier) {
            let list = events_by_context_id.get_mut(&context_identifier).unwrap();
            list.push(log_event);
        } else {
            let log_events_by_key = vec!(log_event);
            events_by_context_id.insert(context_identifier, log_events_by_key);
        }
    }

    AggregatedLogs {
        log_filename: log_results.log_filename,
        events_by_context_id: events_by_context_id
    }
}
