use std::env;
use std::error::Error;
mod config;
use crate::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("CONFIG FILE: {}", &args[1]);

    let config: Config = Config::read_config(&args[1])?;
    println!("YAML: {:?}", config);

    return Ok(());
}
