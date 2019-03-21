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

    printer!("Ryugu's Eccentricity Vector", v => ryugu.eccentricity_vec());
    printer!("Ryugu's Semi-Major Axis", s => ryugu.semi_major_axis());
    printer!("Ryugu's True Anomaly", s => ryugu.true_anomaly());
    printer!("Ryugu's Orbital Period", s => ryugu.orbital_period() * SECTODAY);
    printer!("Ryugu's Eccentric Anomaly", s => ryugu.eccentric_anomaly());
    printer!("Time Since Periapsis", s => ryugu.time_since_periapsis() * SECTODAY);
}
