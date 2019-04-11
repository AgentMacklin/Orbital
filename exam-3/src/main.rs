#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 */
use nalgebra::Vector3;
const SOLARGM: f64 = 2.963092749241593e-4;

// extern crate colored;
use colored::*;

#[macro_use]
mod macros;
mod body;
mod date;
use body::Body;

const DAYTOSEC: f64 = 24.0 * 3600.0;

fn main() {
    let pluto = Body::new(
        Vector3::new(
            1.218193989126378e1,
            -3.149522235231989e1,
            -1.535562041975234e-1,
        ),
        Vector3::new(
            3.000627734261702e-3,
            4.635059607321797e-4,
            -9.300258803000724e-4,
        ),
    );

    let neptune = Body::new(
        Vector3::new(
            2.905640909261118e1,
            -7.174984730218214e0,
            -5.218791016710037e-1,
        ),
        Vector3::new(
            7.317748743401405e-4,
            3.065897473349852e-3,
            -8.039332012516184e-5,
        ),
    );

    let julian = 2458584.50000;
    let greg_date = date!(julian);

    let time = 10_000.352;

    let mut day = 0.0;
    let mut neptune_radius = neptune.position_at_time(day).norm();
    let mut pluto_radius = pluto.position_at_time(day).norm();

    // Keep incrementing the julian day by one day until pluto is closer than neptune
    println!("\nCalculating the date when Pluto pass Neptune's orbit, this will take a bit...");
    while neptune_radius < pluto_radius {
        day += 1_f64;
        neptune_radius = neptune.position_at_time(day).norm();
        pluto_radius = pluto.position_at_time(day).norm();
    }
    println!("Done!");

    let future_greg = date!(day + julian);

    printer!("G", s => neptune.semi_major_axis());
    printer!("H", s => neptune.eccentricity());
    printer!("I", s => neptune.inclination().to_degrees());
    printer!("J", s => neptune.argument_of_periapsis().to_degrees());
    printer!("K", s => neptune.argument_of_ascending_node().to_degrees());
    printer!("L", s => neptune.true_anomaly().to_degrees());
    printer!("M", s => pluto.semi_major_axis());
    printer!("N", s => pluto.eccentricity());
    printer!("O", s => pluto.inclination().to_degrees());
    printer!("P", s => pluto.argument_of_periapsis().to_degrees());
    printer!("Q", s => pluto.argument_of_ascending_node().to_degrees());
    printer!("R", s => pluto.true_anomaly().to_degrees());
    printer!("S-T-U", v => neptune.position_at_time(time));
    printer!("V-W-Z", v => neptune.velocity_at_time(time));
    printer!("Y-Z-AA", v => pluto.position_at_time(time));
    printer!("AB-AC-AD", v => pluto.velocity_at_time(time));
    println!("Date when Pluto is closer than Neptune:\n{}", future_greg);
}
