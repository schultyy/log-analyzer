use log_reader::LogEvent;
use config::ConfigFile;
use std::collections::HashMap;

use aggregator;
use config;
use log_reader;

#[derive(Debug, Serialize)]
pub struct ValidationResults {
    log_filename: String,
    validation_results: HashMap<String, HashMap<String, bool>>
}

pub fn validate_workflow_for_file(aggregated_logs: aggregator::AggregatedLogs, config_file: &config::ConfigFile) -> ValidationResults {
    let mut validation_results : HashMap<String, HashMap<String, bool>> = HashMap::new();
    for (context_identifier, log_events) in aggregated_logs.events_by_context_id {
        validation_results.insert(context_identifier, validate_single(&log_events, &config_file));
    }

    ValidationResults {
        log_filename: aggregated_logs.log_filename,
        validation_results: validation_results
    }
}

pub fn validate_workflow_for_single_context_id(log_filename: String, log_events: &Vec<log_reader::LogEvent>, config_file: &config::ConfigFile) -> ValidationResults {
    let validation_results_for_single_id = validate_single(log_events, &config_file);

    let mut validation_results = HashMap::new();

    if log_events.len() > 0 {
        validation_results.insert(log_events[0].context_identifier.to_string(), validation_results_for_single_id);
    }

    ValidationResults {
        log_filename: log_filename,
        validation_results: validation_results
    }
}

fn validate_single(log_events: &Vec<LogEvent>, config: &ConfigFile) -> HashMap<String, bool> {
    let mut check_list = HashMap::new();

    for config_step in &config.steps {
        check_list.insert(config_step.name.clone(), log_events.iter().find(|event| event.name == config_step.name).is_some());
    }

    check_list
}
