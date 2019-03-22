#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 */
extern crate nalgebra;
use nalgebra::{Matrix3, Vector3};

extern crate colored;
use colored::*;

#[macro_use]
mod util;
use self::util::*;
use std::f64::consts::PI;

const SOLARGM: f64 = 1.328905188132376e20;

const AUTOMETER: f64 = 1.49597870700e11;
const DAYTOSEC: f64 = 24.0 * 3600.0;
const TOMETRIC: f64 = DAYTOSEC / AUTOMETER ;

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
    
   /**
    * Creating B frame
    */
    let b_a = new_horizons.position.normalize();
    let b_b = new_horizons.velocity.normalize();
    let b_c = b_a.cross(&b_b);

    let b_frame = Matrix3::from_rows(&[b_a.transpose(), b_b.transpose(), b_c.transpose()]);
    let b_frame_inverse = b_frame.try_inverse().expect("Could not invert B frame matrix.");
    let v_b = Vector3::new(new_horizons.velocity.dot(&b_b), 0.0, 0.0);
    
    printer!("A-B-C", v => f_frame.row(0));
    printer!("D-E-F", v => f_frame.row(1));
    printer!("G-H-I", v => f_frame.row(2));
    printer!("J-K-L", v => v_b);
    printer!("M-N-O", v => b_frame_inverse * v_b);
    printer!("P-Q-R", v => new_horizons.eccentricity_vec());
    printer!("S-T-U", v => f_frame * new_horizons.eccentricity_vec());
    printer!("V", s => new_horizons.angular_momentum().norm());
}
