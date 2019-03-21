#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 */
extern crate nalgebra;
use nalgebra::Vector3;

extern crate colored;
use colored::*;

#[macro_use]
mod util;
use self::util::*;
use std::f64::consts::PI;

const METERTOAU: f64 = 1.0 / 1.49597870700e11;
const SECTODAY: f64 = 1.0 / (24.0 * 3600.0);
const TODAYSEC: f64 = (24.0 * 3600.0) * (METERTOAU * METERTOAU);
const KMTOM: f64 = 1000_f64;

fn main() {
    let ryugu = Body {
        position: Vector3::new(
            1.132759321672478E+08,
           -1.733831194873283E+08,
            1.676906176312257E+07),
        velocity: Vector3::new(
            1.804598259087825E+01,
            1.405146782685940E+01,
            1.309063084989150E+00)
    };

    let delta_e = |E: f64, nt: f64, eccen: f64| (E - eccen * E.sin() - nt) / (1.0 - eccen * E.cos());
    let kepler = |E: f64, nt: f64, eccen: f64| E - delta_e(E, nt, eccen);

    let nt_init = PI / 3.0;
    let eccen_init = 0.3;
    let mut E_0 = 0.0;

    printer!("\nDelta-E Naught", s => delta_e(E_0, nt_init, eccen_init));
    let mut E = kepler(E_0, nt_init, eccen_init);

    printer!("First Iteration", s => E);
    E_0 = E;
    E = kepler(E_0, nt_init, eccen_init);
    printer!("Second Iteration", s => E);

    // Completing the rest of the iterations, with a tolerance of 10^-12 since Rust is fast
    while (E - E_0).abs() > 1.0e-12 {
        E_0 = E;
        E = kepler(E_0, PI / 3.0, 0.3);
    }

    /* 1b */
    let mut F_0 = 0.0;
    let nt = (3.0 * PI) / 4.0;
    let eccen = 1.73;
    let delta_e_rev = |F: f64, nt: f64, eccen: f64| (eccen * F.sinh() + nt - F) / (eccen * F.cosh() - 1.0);
    let kepler_rev = |F: f64, nt: f64, eccen: f64| F + delta_e_rev(F, nt, eccen);

    printer!("Delta-F Naught", s => delta_e(F_0, nt, eccen));
    let mut F = kepler(F_0, nt, eccen);

    printer!("First Iteration", s => F);
    F_0 = F;
    F = kepler(F_0, nt, eccen);
    printer!("Second Iteration", s => F);

    // Completing the rest of the iterations, with a tolerance of 10^-12 since Rust is fast
    while (F - F_0).abs() > 1.0e-12 {
        F_0 = F;
        F = kepler(E_0, PI / 3.0, 0.3);
    }
    printer!("Converged E", s => E);
    printer!("Converged F", s => F);
    printer!("Ryugu's Eccentricity Vector", v => ryugu.eccentricity_vec());
    printer!("Ryugu's Semi-Major Axis", s => ryugu.semi_major_axis());
    printer!("Ryugu's True Anomaly", s => ryugu.true_anomaly());
    printer!("Ryugu's Orbital Period", s => ryugu.orbital_period() * SECTODAY);
    printer!("Ryugu's Eccentric Anomaly", s => -ryugu.eccentric_anomaly() + 360.0);
    printer!("Time Since Periapsis", s => ryugu.time_since_periapsis() * SECTODAY);
}
