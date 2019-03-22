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

const SOLARGM: f64 = 1.328905188132376e20;

const AUTOMETER: f64 = 1.49597870700e11;
const DAYTOSEC: f64 = 24.0 * 3600.0;
const TOMETRIC: f64 = DAYTOSEC / AUTOMETER;

fn main() {
    let new_horizons = Body {
        position: Vector3::new(
            1.229648731159843e1,
            -4.213191995271971e1,
            1.498733149080106e0,
        ) * AUTOMETER,
        velocity: Vector3::new(
            3.155768655215773e3,
            -7.479676821277172e3,
            2.933668481642173e4,
        ) * TOMETRIC,
    };

    let f_frame = new_horizons.make_frame();

    printer!("A-B-C", v => f_frame.row(0));
    printer!("D-E-F", v => f_frame.row(1));
    printer!("G-H-I", v => f_frame.row(2));
}
