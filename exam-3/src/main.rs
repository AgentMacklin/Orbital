#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 ** Austen LeBeau
 ** ENGR 3310-002
 **/
use nalgebra::Vector3;
const SOLARGM: f64 = 2.963092749241593e-4;

// extern crate colored;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

#[macro_use]
mod macros;
mod body;
mod date;
use body::Body;

const DAYTOSEC: f64 = 24.0 * 3600.0;

fn main() {
    // Style for progress bars, it's really stupid but I like it
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] [{eta_precise}]")
        .progress_chars("#>-");

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

    /**
     * CALCULATING THE DAY WHEN PLUTO IS CLOSER THAN NEPTUNE
     */
    let mut day = 50000.0 + time;
    let mut neptune_radius = neptune.position_at_time(day).norm();
    let mut pluto_radius = pluto.position_at_time(day).norm();

    // Keep incrementing the julian day by one day until pluto is closer than
    // neptune
    println!("\nCalculating the date when Pluto passes Neptune's orbit, this will take a bit...");
    while neptune_radius < pluto_radius {
        day += 1_f64;
        neptune_radius = neptune.position_at_time(day).norm();
        pluto_radius = pluto.position_at_time(day).norm();
    }
    println!("Done!");

    let first_date = day + julian;

    /**
     * CALCULATING WHEN PLUTO PASSES NEPTUNE AGAIN
     */
    let mut neptune_radius = neptune.position_at_time(day).norm();
    let mut pluto_radius = pluto.position_at_time(day).norm();

    println!("Calculating the date when Neptune passes Plutos's orbit...");
    while neptune_radius > pluto_radius {
        day += 1_f64;
        neptune_radius = neptune.position_at_time(day).norm();
        pluto_radius = pluto.position_at_time(day).norm();
    }
    println!("Done!");

    let second_date = day + julian;

    /**
     * CALCULATING THE DATE WHEN THE DISTANCE BETWEEN PLUTO AND NEPTUNE IS
     * THE SHORTEST
     */
    println!("Calculating closest approach of Neptune and Pluto...");
    let distance_pb = ProgressBar::new(500 * 365);
    distance_pb.set_style(style.clone());
    let mut distance = neptune.distance_to(&pluto);
    let mut min_distance = distance;
    let mut min_day = 0;
    for i in 1..(500 * 365) {
        let new_pluto = Body::new(
            pluto.position_at_time(i as f64),
            pluto.velocity_at_time(i as f64),
        );
        let new_neptune = Body::new(
            neptune.position_at_time(i as f64),
            neptune.velocity_at_time(i as f64),
        );
        distance = new_neptune.distance_to(&new_pluto);
        if distance < min_distance {
            min_distance = distance;
            min_day = i;
        }
        distance_pb.inc(1);
    }
    distance_pb.finish_with_message("\nDone!\n");

    println!(
        "{}\n{}\n",
        macros::underline("A-B-C-D-E-F").cyan(),
        date!(julian)
    );
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
    println!(
        "{}\n{}\n",
        macros::underline("AE-AF-AG").cyan(),
        date!(first_date)
    );
    printer!("AH-AI-AJ", v => neptune.position_at_time(first_date - julian));
    printer!("AK-AL-AM", v => pluto.position_at_time(first_date - julian));
    println!(
        "{}\n{}\n",
        macros::underline("AN-AO-AP").cyan(),
        date!(second_date)
    );
    printer!("AQ-AR-AS", v => neptune.position_at_time(second_date - julian));
    printer!("AK-AL-AM", v => pluto.position_at_time(second_date - julian));
    println!(
        "{}\n{}\n",
        macros::underline("Date of Closest Approach").cyan(),
        date!((min_day as f64) + julian)
    );
}
