use crate::srclist::*;
use marlu::RADec;
use ndarray::prelude::*;
use ndarray_linalg::*;
use num_complex::*;
use rayon::prelude::*;
use std::f64::consts::PI;

pub fn calculate_crb(
    baselines_xy: &Array<f64, Dim<[usize; 3]>>,
    source_list: ComponentList,
    lambda: f64,
    freq: f64,
    sigma: f64,
    phase_centre: RADec,
) {
    let i = Complex::new(0.0, 1.0);
    let mut crb = Array::<Complex64, _>::zeros((128, 128));

    let baselines = baselines_xy / lambda;

    for a in 0usize..128usize {
        println!("{}", a);
        // (0usize..128usize).into_par_iter().for_each(|a| {
        for b in a..128usize {
            for src_i in source_list.iter() {
                for src_j in source_list.iter() {
                    // TODO: Need to move these calculations outside
                    let B_i = src_i.estimate_at_freq(freq).i;
                    let B_j = src_j.estimate_at_freq(freq).i;
                    let lmn_i = src_i.radec.to_lmn(phase_centre);
                    let lmn_j = src_j.radec.to_lmn(phase_centre);

                    let l_i = lmn_i.l;
                    let m_i = lmn_i.m;
                    let l_j = lmn_j.l;
                    let m_j = lmn_j.m;
                    let complex_part: Complex64 = (2.0
                        * PI
                        * i
                        * (baselines[[a, b, 0]] * (l_i - l_j)
                            + baselines[[a, b, 1]] * (m_i - m_j)))
                        .exp();

                    crb[[a, b]] = B_i * B_j * complex_part;

                    if a == b {
                        crb[[a, b]] += 131.0 * B_i * B_j;
                    }
                }
            }
        }
    }

    for a in 0usize..128usize {
        for b in a..128usize {
            crb[[b, a]] = crb[[a, b]].conj();
        }
    }

    crb = Complex::new(2.0 / sigma.powi(2), 0.0) * crb;
}
