use log_reader::LogEvent;
use config::ConfigFile;
use std::collections::HashMap;

pub fn validate_single(log_events: &Vec<LogEvent>, config: &ConfigFile) -> HashMap<String, bool> {
    let mut check_list = HashMap::new();

    for config_step in &config.steps {
        check_list.insert(config_step.name.clone(), log_events.iter().find(|event| event.name == config_step.name).is_some());
    }

    check_list
}
