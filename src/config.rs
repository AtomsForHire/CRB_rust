use serde::Deserialize;
use std::error::Error;
use std::{fs, path::Path};

// Struct to store input parameters
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ra: f64,
    pub dec: f64,
    pub T_sys: f64,
    pub D: f64,
    pub channel_width: f64,
    pub start_freq: f64,
    pub end_freq: f64,
    pub int_time: f64,
    pub srclist: String,
    pub metafits: String,
    pub output: String,
    pub telescope: String,
}

impl Config {
    // Read in input parameters for config YAML file
    pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file_str: String = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&file_str)?;
        return Ok(config);
    }
}
