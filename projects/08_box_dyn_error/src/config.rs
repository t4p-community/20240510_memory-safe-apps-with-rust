use serde::Deserialize;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub some_api_key: String,
    pub log_level: String,
    pub log_file: String,
}

impl Config {
    // The "Box<dyn std::error::Error>" return type is a trait object that can represent
    // any type that implements the Error trait.
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Self = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "API Key: {}\nLog Level: {}\nLog File: {}",
            self.some_api_key, self.log_level, self.log_file
        )
    }
}
