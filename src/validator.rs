use log_reader::LogEvent;
use config::ConfigFile;

pub fn validate_single(log_events: &Vec<LogEvent>, config: &ConfigFile) -> Result<(), String> {
    if config.steps.len() != log_events.len() {
        return Err(format!("Config mandates {} steps, log events has only {} elements", config.steps.len(), log_events.len()))
    }
    Ok(())
}