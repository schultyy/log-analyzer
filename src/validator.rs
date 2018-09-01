use log_reader::LogEvent;
use config::ConfigFile;
use std::collections::HashMap;

use aggregator;
use config;
use log_reader;

pub fn validate_workflow_for_file(aggregated_logs: aggregator::AggregatedLogs, config_file: &config::ConfigFile) -> HashMap<String, HashMap<String, bool>> {
    let mut validation_results : HashMap<String, HashMap<String, bool>> = HashMap::new();
    for (context_identifier, log_events) in aggregated_logs.events_by_context_id {
        validation_results.insert(context_identifier, validate_single(&log_events, &config_file));
    }
    validation_results

}

pub fn validate_workflow_for_single_context_id(log_events: &Vec<log_reader::LogEvent>, config_file: &config::ConfigFile) -> HashMap<String, bool> {
    validate_single(log_events, &config_file)
}

pub fn validate_single(log_events: &Vec<LogEvent>, config: &ConfigFile) -> HashMap<String, bool> {
    let mut check_list = HashMap::new();

    for config_step in &config.steps {
        check_list.insert(config_step.name.clone(), log_events.iter().find(|event| event.name == config_step.name).is_some());
    }

    check_list
}
