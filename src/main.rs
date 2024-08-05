use std::env;
use std::error::Error;
use std::fs;
use std::process;
use yaml_rust2::{YamlEmitter, YamlLoader};

#[derive(Debug)]
struct Config {
    ra: f64,
    dec: f64,
    t_sys: f64,
    d: f64,
    channel_width: f64,
    start_freq: f64,
    end_freq: f64,
    int_time: f64,
    srclist: String,
    metafits: String,
    output: String,
    telescope: String,
}

fn read_config(filename: &String) -> Result<Config, Box<dyn Error>> {
    let file_str: String = fs::read_to_string(filename)?;
    let yamls = YamlLoader::load_from_str(&file_str).unwrap();
    let yaml = &yamls[0];

    // println!("{}", yaml["channel_width"].as_f64().unwrap());

    let config = Config {
        ra: yaml["ra"].as_f64().unwrap_or_else(|| {
            eprintln!("ra must be a float (include the '.')");
            process::exit(1);
        }),
        dec: yaml["dec"].as_f64().unwrap_or_else(|| {
            eprintln!("dec must be a float (include the '.')");
            process::exit(1);
        }),
        t_sys: yaml["T_sys"].as_f64().unwrap_or_else(|| {
            eprintln!("T_sys must be a float (include the '.')");
            process::exit(1);
        }),
        // t_sys: 400.0,
        d: yaml["D"].as_f64().unwrap_or_else(|| {
            eprintln!("D must be a float (include the '.')");
            process::exit(1);
        }),
        channel_width: yaml["channel_width"].as_f64().unwrap_or_else(|| {
            eprintln!("Something wrong with channel_width value");
            process::exit(1);
        }),
        start_freq: yaml["start_freq"].as_f64().unwrap_or_else(|| {
            eprintln!("Something wrong with start_freq input value");
            process::exit(1);
        }),
        end_freq: yaml["end_freq"].as_f64().unwrap_or_else(|| {
            eprintln!("Something wrong with end_freq input value");
            process::exit(1);
        }),
        int_time: yaml["int_time"].as_f64().unwrap_or_else(|| {
            eprintln!("int_time must be a float (include the '.')");
            process::exit(1);
        }),
        srclist: yaml["srclist"].clone().into_string().unwrap_or_else(|| {
            eprintln!("Something wrong with srclist string in config");
            process::exit(1);
        }),

        metafits: yaml["metafits"].clone().into_string().unwrap_or_else(|| {
            eprintln!("Something wrong with metafits string in config");
            process::exit(1);
        }),

        telescope: yaml["telescope"].clone().into_string().unwrap_or_else(|| {
            eprintln!("Something wrong with telescope string in config");
            process::exit(1);
        }),

        output: yaml["output"].clone().into_string().unwrap_or_else(|| {
            eprintln!("Something wrong with output string in config");
            process::exit(1);
        }),
    };

    return Ok(config);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("CONFIG FILE: {}", &args[1]);
    let config: Config = read_config(&args[1])?;

    println!("YAML: {:?}", config);
    return Ok(());
}
