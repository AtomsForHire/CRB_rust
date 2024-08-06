use serde::Deserialize;
use std::error::Error;
use std::{fs, path::Path};

// Struct to store input parameters
#[derive(Debug, Deserialize)]
pub struct Config {
    ra: f64,
    dec: f64,
    T_sys: f64,
    D: f64,
    channel_width: f64,
    start_freq: f64,
    end_freq: f64,
    int_time: f64,
    srclist: String,
    metafits: String,
    output: String,
    telescope: String,
}

impl Config {
    // Read in input parameters for config YAML file
    pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file_str: String = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&file_str)?;
        return Ok(config);
    }
}
