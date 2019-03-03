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

const AUTOM: f64 = 1.49597870700E11;
const DAYTOSEC: f64 = 86400_f64;
const TOMETRIC: f64 = AUTOM / DAYTOSEC;

fn main() {
    let earth = Body {
        position: Vector3::new(
            -8.461345399508943E-01,
            5.198188201638625E-01,
            -6.874116231359140E-05,
        ) * AUTOM,
        velocity: Vector3::new(
            -9.202068150470241E-03,
            -1.477025937149794E-02,
            2.181018061038459E-07,
        ) * TOMETRIC,
    };

    let venus = Body {
        position: Vector3::new(
            -5.728936053119389E-01,
            -4.341301111844528E-01,
            2.688719930686330E-02,
        ) * AUTOM,
        velocity: Vector3::new(
            1.221413056525722E-02,
            -1.610029029497521E-02,
            -9.260442405719175E-04,
        ) * TOMETRIC,
    };

    let e_r = earth.position.normalize();
    let e_h = earth.angular_momentum().normalize();
    let e_tht = e_h.cross(&e_r);

    let e_1 = Vector3::new(1_f64, 0_f64, 0_f64);
    let e_2 = Vector3::new(0_f64, 1_f64, 0_f64);
    let e_3 = Vector3::new(0_f64, 0_f64, 1_f64);

    let i_units = [e_r, e_tht, e_h];
    let r_units = [e_1, e_2, e_3];

    let trans_mat = transform(i_units, r_units);

    let rel_position = trans_mat * (earth.position - venus.position);
    let rel_veloc = trans_mat * earth.relative_velocity(&venus);

    printer!("\nEarth's Total Energy", s => earth.total_energy());
    printer!("Earth's Angular Momentum", v => earth.angular_momentum());
    printer!("Earth's Radial Velocity", v => earth.radial_veloc());
    printer!("Earth's Tangential Velocity", v => earth.tangential_veloc());
    printer!("Earth's Eccentricity Vector", v => earth.eccentricity_vec());
    printer!("Earth's True Anomaly", s => earth.true_anomaly());
    printer!("Frame Rotation Rate", s => earth.rotation_rate());
    printer!("e_r", v => e_r);
    printer!("e_tht", v => e_tht);
    printer!("e_h", v => e_h);
    printer!("Venus' Relative Position", v => rel_position);
    printer!("Venus' Relative Velocity", v => rel_veloc);
    printer!("Earth-Venus Angle", s => earth.angle_to(&venus));
    printer!("Venus' Eccentricity Vector", v => venus.eccentricity_vec());
    printer!("Venus' True Anomaly", s => venus.true_anomaly());
    printer!("Transformation Matrix", m => trans_mat);
}
