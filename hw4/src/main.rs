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
const DAYTOSEC: f64 = 24.0 * 3600.0;

fn main() {
    let ryugu = Body {
        position: Vector3::new(
            1.132759321672478E+08,
            -1.733831194873283E+08,
            1.676906176312257E+07,
        ),
        velocity: Vector3::new(
            1.804598259087825E+01,
            1.405146782685940E+01,
            1.309063084989150E+00,
        ),
    };

    /* PROBLEM 1a */
    let mut nt = PI / 3.0;
    let mut eccen = 0.3;

    printer!("\nA", s => -delta_e(0.0, nt, eccen));

    let b = kepler(0.0, nt, eccen);
    printer!("B", s => b);
    printer!("C", s => -delta_e(b, nt, eccen));
    printer!("D", s => kepler_iterate(b, nt, eccen));

    /* PROBLEM 1a v. */
    nt = (17.0 * PI) / 4.0;
    printer!("E", s => -delta_e(0.0, nt, eccen));

    let f = kepler(0.0, nt, eccen);
    printer!("F", s => f);
    printer!("G", s => -delta_e(f, nt, eccen));
    printer!("H", s => kepler_iterate(f, nt, eccen));

    /* PROBLEM 1b */
    nt = (3.0 * PI) / 4.0;
    eccen = 1.73;
    printer!("I", s => -hyper_delta_e(0.0, nt, eccen));

    let j = hyper_kepler(0.0, nt, eccen);
    printer!("J", s => j);
    printer!("K", s => -hyper_delta_e(j, nt, eccen));
    printer!("L", s => hyper_kepler_iterate(j, nt, eccen));

    nt = (13.0 * PI) / 3.0;
    printer!("M", s => -hyper_delta_e(0.0, nt, eccen));
    let n = hyper_kepler(0.0, nt, eccen);
    printer!("N", s => n);
    printer!("O", s => hyper_kepler_iterate(n, nt, eccen));

    nt = 76.0 * PI;
    printer!("P", s => hyper_kepler_iterate(0.0, nt, eccen));

    nt = 194.0 * PI;
    let q = hyper_kepler_iterate(5.0, nt, eccen);
    printer!("Q", s => q);

    let eccen_anom = ryugu.eccen_anom_at_time(143.0 * DAYTOSEC).to_radians();

    let sma = ryugu.semi_major_axis().to_radians();
    let e_vec = ryugu.eccentricity_vec().normalize();

    let x = (sma * eccen_anom.cos()) - (sma * e_vec.norm());
    let y = sma * (1.0 - e_vec.norm().powi(2)).sqrt() * eccen_anom.sin();

    let e_h = ryugu.angular_momentum().normalize();
    let e_t = e_h.cross(&e_vec);

    let radius = x * e_vec + y * e_t;

    /**
     * The rest of the homework
     */
    printer!("Ryugu's Eccentricity Vector", v => ryugu.eccentricity_vec());
    printer!("Ryugu's Semi-Major Axis", s => ryugu.semi_major_axis());
    printer!("Ryugu's True Anomaly", s => ryugu.true_anomaly());
    printer!("Ryugu's Orbital Period", s => ryugu.orbital_period() * SECTODAY);
    printer!("Ryugu's Eccentric Anomaly", s => ryugu.eccentric_anomaly());
    printer!("Time Since Periapsis", s => ryugu.time_since_periapsis() * SECTODAY);
    printer!("Eccentric Anomaly (143 days)", s => eccen_anom);
    printer!("Last Problem", v => radius);
}
