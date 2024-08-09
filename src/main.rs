mod srclist;
use marlu::{RADec, LMN};
use std::env;
use std::error::Error;
use std::io;
mod config;
use crate::config::Config;
use crate::srclist::read;
use physical_constants;
use srclist::*;
use std::fs;

fn calc_rms(T_sys: f64, bandwith: f64, int_time: f64, telescope: &String) -> f64 {
    let mut A_eff: f64 = 0.0;

    if telescope == "mwa" {
        A_eff = 21.0;
    } else if telescope == "ska" {
        A_eff = (35.0f64 / 2.0f64).powi(2);
    }

    let k = physical_constants::BOLTZMANN_CONSTANT;

    return 10.0f64.powi(26) * (2.0 * k * T_sys) / (A_eff * (bandwith * int_time).sqrt());
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("CONFIG FILE: {}", &args[1]);

    let config: Config = Config::read_config(&args[1])?;
    println!("YAML: {:?}", config);

    let file = fs::File::open(&config.srclist)?;
    let mut buf_reader = io::BufReader::new(file);
    let source_list: SourceList = read::source_list_from_yaml(&mut buf_reader)?;

    let mut component_list: ComponentList = ComponentList::new(source_list);

    println!(
        "Number of components before veto: {}",
        &component_list.len()
    );

    let rms = calc_rms(
        config.T_sys,
        config.channel_width,
        config.int_time,
        &config.telescope,
    );

    component_list.veto_by_flux(rms);
    println!(
        "Number of components after flux {} veto: {}",
        rms,
        &component_list.len()
    );

    let phase_centre = RADec {
        ra: config.ra,
        dec: config.dec,
    };

    component_list.veto_by_fov(phase_centre, 2.0, config.D);
    println!("Number of components after veto: {}", &component_list.len());

    println!("{:?}", config);

    return Ok(());
}
