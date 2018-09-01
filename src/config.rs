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

pub fn read_config_from_file(path: &str) -> Result<ConfigFile, Box<Error>> {
    let canonical_path = Path::new(&path).canonicalize()?;
    let file = File::open(canonical_path)?;
    let config_file = serde_json::from_reader(file)?;
    Ok(config_file)
}