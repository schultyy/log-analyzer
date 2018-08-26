use config::{ConfigFile, ConfigStep};
use regex::Regex;

#[derive(Debug)]
pub struct LogEvent {
    pub context_identifier: String,
    pub name: String,
    pub payload: Vec<String>
}

fn matches_context_argument(log_line: &str, compiled_context_expressions: &Vec<Regex>) -> Option<String> {
    for compiled_context_expression in compiled_context_expressions {
        if compiled_context_expression.is_match(log_line) {
            for cap in compiled_context_expression.captures_iter(log_line) {
                return Some(cap[1].to_string())
            }
        }
    }
    None
}

fn extract_payload(log_line: &str, config_step: &ConfigStep) -> Vec<String> {
    let mut collected_payload = vec!();

    for payload_item in &config_step.payload {
        let compiled_payload_item = Regex::new(&payload_item).unwrap();
        for capture in compiled_payload_item.captures_iter(log_line) {
            collected_payload.push(capture[0].to_string());
        }
    }
    collected_payload
}

fn match_step_identifier(context_value: String, log_line: &str, config_file: &ConfigFile) -> Option<LogEvent> {
    for config_file_step in config_file.steps.iter() {
        let regex = Regex::new(&config_file_step.identifier).unwrap();
        if regex.is_match(log_line) {
            return Some(LogEvent {
                context_identifier: context_value,
                name: config_file_step.name.clone(),
                payload: extract_payload(log_line, &config_file_step)
            })
        }
    }
    None
}

pub fn extract(config_file: ConfigFile, log_file: String) -> Vec<LogEvent> {
    let mut log_events = Vec::new();
    let log_file_lines = log_file.split("\n");
    let compiled_context_expressions = config_file.context_arguments.iter().map(|context| Regex::new(&context).unwrap()).collect();

    for log_line in log_file_lines {
        if let Some(context_value) = matches_context_argument(&log_line, &compiled_context_expressions) {
            if let Some(payload) = match_step_identifier(context_value, log_line, &config_file) {
                log_events.push(payload);
            }
        }
    }

    log_events
}