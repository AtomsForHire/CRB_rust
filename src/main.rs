mod calc;
mod srclist;
use calc::*;
use marlu::RADec;
use std::env;
use std::error::Error;
use std::io;
mod config;
use crate::config::Config;
use crate::srclist::read;
use mwalib::MetafitsContext;
use ndarray::prelude::*;
use physical_constants;
use srclist::*;
use std::fs;
use std::time::Instant;

fn create_baselines(metafits: &String) -> Result<Array<f64, Dim<[usize; 3]>>, Box<dyn Error>> {
    let file = MetafitsContext::new(metafits, None)?;

    let mut baselines_xy = Array::<f64, _>::zeros((128, 128, 2));

    for (i, ant_i) in file.antennas.iter().enumerate() {
        for (j, ant_j) in file.antennas.iter().enumerate() {
            baselines_xy[[i, j, 0]] = ant_i.east_m - ant_j.east_m;
            baselines_xy[[i, j, 1]] = ant_i.north_m - ant_j.north_m;
        }
    }
    return Ok(baselines_xy);
}

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

    let rms_re = calc_rms(
        config.T_sys,
        config.channel_width,
        config.int_time,
        &config.telescope,
    );

    let rms_vis: f64 = 5.0 * (rms_re / 8256.0f64.sqrt());
    component_list.veto_by_flux(rms_vis);

    println!(
        "Number of components after flux {} veto: {}",
        rms_vis,
        &component_list.len()
    );

    // Create phase centre from inputs
    let phase_centre = RADec::from_degrees(config.ra, config.dec);
    println!("phase centre: {:?}", phase_centre);

    // for comp in component_list.iter() {
    //     let lmn = comp.radec.to_lmn(phase_centre);
    //     println!(
    //         "ra: {}, dec: {}, l: {}, m: {}, dist: {}",
    //         comp.radec.ra,
    //         comp.radec.dec,
    //         lmn.l,
    //         lmn.m,
    //         (lmn.l.powi(2) + lmn.m.powi(2)).sqrt()
    //     );
    // }

    let baselines_xy = create_baselines(&config.metafits)?;
    // println!("{:?}", baselines_xy);

    let num_freq: usize =
        ((config.end_freq - config.start_freq) / config.channel_width).floor() as usize;

    println!("NUM FREQ: {}", num_freq);
    let freq_array = Array::linspace(config.start_freq, config.end_freq, num_freq);

    for freq in freq_array.iter() {
        let start_time = Instant::now();
        println!("==================== FREQ: {} ====================", freq);

        let lambda = physical_constants::SPEED_OF_LIGHT_IN_VACUUM / freq;
        let mut freq_comp_list = component_list.clone();
        freq_comp_list.veto_by_fov(phase_centre, lambda, config.D);
        println!(
            "Number of components after veto: {}",
            &freq_comp_list.slice_to_struct(0..500).len()
        );

        println!("Calculating CRB");
        calc::calculate_crb(
            &baselines_xy,
            freq_comp_list.slice_to_struct(0..500),
            lambda,
            *freq,
            rms_vis,
            phase_centre,
        );

        let end_time = start_time.elapsed();
        println!("This block took: {:?}", end_time);
    }

    return Ok(());
}
