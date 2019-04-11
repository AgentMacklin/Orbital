#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 */
use nalgebra::Vector3;
const SOLARGM: f64 = 1.328905188132376e11;

// extern crate colored;
use colored::*;

#[macro_use]
mod macros;
mod body;
mod date;
use body::{Body, OrbitType};

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

    let time = 10000.35615;

    let current_julian = date!(2019-03-23 20:00:00);
    let new_julian = current_julian + time;
    let greg_date = date!(new_julian);
    let t_anom = earth.true_anomaly_at_time(time);
    let trans_mat = body::three_one_three_transform(
        t_anom + earth.argument_of_periapsis(),
        earth.inclination(),
        earth.argument_of_ascending_node(),
    )
    .try_inverse()
    .unwrap();

    let p = earth.orbital_parameter() / (1.0 + earth.eccentricity() * t_anom.cos());
    let v = (.sqrt();

    let r_B = Vector3::new(p, 0.0, 0.0);
    let v_B = Vector3::new(v * t_anom.cos(), v * t_anom.sin(), 0.0);

    printer!("A-B-C", v => earth.position);
    printer!("D-E-F", v => earth.velocity);
    printer!("G", s => earth.semi_major_axis());
    printer!("H", s => earth.eccentricity());
    printer!("I", s => earth.inclination().to_degrees());
    printer!("J", s => earth.argument_of_periapsis().to_degrees());
    printer!("K", s => earth.argument_of_ascending_node().to_degrees());
    printer!("L", s => t_anom.to_degrees());
    println!("{}\n{}\n", macros::underline("Problem 8").cyan(), greg_date);
    printer!("Problem 9", m => trans_mat);
    printer!("Position", v => trans_mat * r_B);
    printer!("Velocity", v => trans_mat * v_B);
}
