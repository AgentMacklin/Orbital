#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 */
use nalgebra::{Matrix3, Vector3};

// extern crate colored;
use colored::*;

#[macro_use]
mod macros;
mod body;
mod date;
use body::Body;

const DAYTOSEC: f64 = 24.0 * 3600.0;

fn main() {
    let earth = Body::new(
        Vector3::new(
            -1.491581119145494E+08,
            -5.727627782875820E+06,
            -5.679400441655191E+03,
        ),
        Vector3::new(
            8.635360877981350E-01,
            -2.985696666561909E+01,
            1.972889032860081E-03,
        ),
    );

    let current_julian = date!(2019-03-23 19:59:59);
    let new_julian = current_julian + 10000.35615;
    let new_julian_secs = new_julian * DAYTOSEC;
    let greg_date = date!(new_julian);

    printer!("A-B-C", v => earth.position);
    printer!("D-E-F", v => earth.velocity);
    printer!("G", s => earth.semi_major_axis());
    printer!("H", s => earth.eccentricity());
    printer!("I", s => earth.inclination());
    printer!("J", s => earth.argument_of_periapsis());
    printer!("K", s => earth.argument_of_ascending_node());
    printer!("L", s => earth.true_anomaly_at_time(new_julian_secs));
    println!(
        "{}\n{}",
        macros::underline("Gregorian Date").cyan(),
        greg_date
    );
    printer!("Test Matrix", m => Matrix3::new(0.0, 0.0, 0.0, 0.0, 0.0, 123100.0, 0.3466, 0.456574, 0.32));
}
