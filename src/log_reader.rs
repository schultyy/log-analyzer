use config::{ConfigFile, ConfigStep};
use regex::Regex;

pub struct LogResults {
    pub name: String,
    pub results: Vec<LogMatch>
}

pub struct LogMatch {
    pub name: String,
    pub matches: Vec<Vec<String>>
}

fn extract_payload_from_line(log_line: &str, payload: &Vec<String>) -> Vec<String> {
    let mut payload_results = vec!();

    for payload_item in payload {
        let re = Regex::new(payload_item).unwrap();
        for capture in re.captures_iter(log_line) {
            payload_results.push(capture[0].to_string());
        }
    }

    payload_results
}

fn execute_step(config_step: &ConfigStep, log_file: &Vec<&str>) -> LogMatch {
    let mut all_matches = vec!();
    let config_identifier_regex = Regex::new(&config_step.identifier).unwrap();
    for line in log_file {
        if config_identifier_regex.is_match(line) {
            let mut new_matches = extract_payload_from_line(line, &config_step.payload);
            all_matches.push(new_matches);
        }
    }

    LogMatch {
        name: config_step.name.clone(),
        matches: all_matches
    }
}

pub fn extract(config_file: ConfigFile, log_file: String) -> LogResults {
    let log_file_lines = log_file.split("\n").collect();
    let mut matches = vec!();
    for config_file_step in config_file.steps.iter() {
        matches.push(execute_step(config_file_step, &log_file_lines));
    }

    LogResults {
        name: config_file.name,
        results: matches
    }
}