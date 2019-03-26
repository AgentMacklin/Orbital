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
const TOMETRIC: f64 = 1731460.0;

fn main() {
    let new_horizons = Body {
        position: Vector3::new(
            1.229648731159843e1,
            -4.213191995271971e1,
            1.498733149080106e0,
        ) * AUTOMETER,
        velocity: Vector3::new(
            3.155768655215773e-3,
            -7.479676821277172e-3,
            2.933668481642173e-4,
        ) * TOMETRIC,
    };

    // let new_horizons = Body {
    //     position: new_horizons.position * AUTOMETER,
    //     velocity: new_horizons.velocity * TOMETRIC
    // };

    let f_frame = new_horizons.make_frame();
    let e_zeta = new_horizons.position.normalize();
    let e_eta = new_horizons.angular_momentum().normalize();
    let e_xi = e_eta.cross(&e_zeta);

    /**
     * Creating B frame
     */
    let b_a = new_horizons.position.normalize();
    let b_b = new_horizons.velocity.normalize();
    let b_c = b_a.cross(&b_b);

    let b_frame = Matrix3::new(
        e_xi[0], e_xi[1], e_xi[2], e_eta[0], e_eta[1], e_eta[2], e_zeta[0], e_zeta[1], e_zeta[2],
    );

    let b_frame_inverse = b_frame
        .try_inverse()
        .expect("Could not invert B frame matrix.");
    let v_b = Vector3::new(new_horizons.velocity.dot(&b_b), 0.0, 0.0);

    let f_anom = (100.0 )

    printer!("A-B-C", v => e_zeta);
    printer!("D-E-F", v => e_eta);
    printer!("G-H-I", v => e_xi);
    printer!("J-K-L", v => v_b);
    // printer!("M-N-O", v => f_frame_inverse * v_b);
    printer!("M-N-O", v => b_frame_inverse * new_horizons.velocity);
    printer!("P-Q-R", v => new_horizons.eccentricity_vec());
    printer!("S-T-U", v => f_frame * new_horizons.eccentricity_vec());
    printer!("V", s => new_horizons.angular_momentum().norm());
    printer!("Semi Major Axis", s => new_horizons.semi_major_axis());
    printer!("X", s => new_horizons.true_anomaly());
    printer!("Y", s => new_horizons.eccentric_anomaly());
    printer!("Z", s => new_horizons.time_since_periapsis() / DAYTOSEC);
    printer!("Z", v => f_frame * new_horizons.eccentricity_vec());
    printer!("Z", v => f_frame * new_horizons.velocity);
}
