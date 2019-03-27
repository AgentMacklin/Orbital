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
mod macros;

mod util;
use self::util::*;

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

    let julian = date!(2019, 3, 23, 8, 0, 0);
    let (year, month, day, hour, minute, second) = date!(julian);

    printer!("A-B-C", v => earth.position);
    printer!("D-E-F", v => earth.velocity);
    printer!("G", s => earth.semi_major_axis());
    printer!("H", s => earth.eccentricity());
    printer!("I", s => earth.inclination());
    printer!("J", s => earth.argument_of_periapsis());
    printer!("K", s => earth.argument_of_ascending_node());
    printer!("Ascending Node", v => earth.ascending_node());
    printer!("Julian Date", s => julian);

}
