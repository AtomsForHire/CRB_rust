mod srclist;
use std::env;
use std::error::Error;
use std::io;
mod config;
use crate::config::Config;
use crate::srclist::read;
use srclist::*;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("CONFIG FILE: {}", &args[1]);

    let config: Config = Config::read_config(&args[1])?;
    println!("YAML: {:?}", config);

    let file = fs::File::open(&config.srclist)?;
    let mut buf_reader = io::BufReader::new(file);
    let source_list: SourceList = read::source_list_from_yaml(&mut buf_reader)?;

    let mut component_list: ComponentList = ComponentList::new(source_list);

    for comp in component_list.iter() {
        println!("{:?}", comp);
    }

    return Ok(());
}
