#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 */
use nalgebra::Vector3;

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

    let julian = 2458566.333333330;
    let julian_test = date!(2019, 3, 23, 19, 59, 59);
    let greg_date = date!(julian_test);

    printer!("A-B-C", v => earth.position);
    printer!("D-E-F", v => earth.velocity);
    printer!("G", s => earth.semi_major_axis());
    printer!("H", s => earth.eccentricity());
    printer!("I", s => earth.inclination());
    printer!("J", s => earth.argument_of_periapsis());
    printer!("K", s => earth.argument_of_ascending_node());
    printer!("Julian Date", s => julian);
    printer!("Julian Test", s => julian_test);
    println!("{}", greg_date);
}
