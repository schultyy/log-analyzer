use std::error::Error;
use std::fs::File;
use std::path::Path;

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub name: String,
    pub context_arguments: Vec<String>,
    pub date_identifier: String,
    pub steps: Vec<ConfigStep>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigStep {
    pub name: String,
    pub identifier: String,
    pub payload: Vec<String>
}

pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<ConfigFile, Box<Error>> {
    let file = File::open(path)?;
    let config_file = serde_json::from_reader(file)?;
    Ok(config_file)
}