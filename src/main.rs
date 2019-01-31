/**
 * Austen LeBeau
 * Due: February 3rd, 2019
 * Main entry point
 */
extern crate nalgebra as na;
use na::Vector3;

#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

mod get_angle;
mod orbitable;

use self::get_angle::get_angle;
use self::orbitable::Body;
// use self::get_angle::get_angle;

fn main() {
    // Pretty print data to a table so it's easier to read
    let mut data_table = Table::new();
    data_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    data_table.set_titles(row![Fg =>
        "Planet",
        "Surface Gravity",
        "Surface Gravity w/ Rotation"
    ]);

    // An array of planets
    let planets = vec![
        Body::new("Mercury", 22032.09, 2440.0, 58.6462),
        Body::new("Venus", 324858.63, 6051.8, -243.0185),
        Body::new("Earth", 398600.440, 6371.01, 0.997257916),
        Body::new("Mars", 42828.3, 3389.9, 1.0274907),
        Body::new("Vesta", 17.8, 262.7, 0.2225886),
        Body::new("Ceres", 62.6284, 470.0, 0.37809041),
        Body::new("Jupiter", 1.26686511e8, 69911.0, 0.413538),
        Body::new("Saturn", 3.79312078e7, 58232.0, 0.4440083),
        Body::new("Uranus", 5.793966e6, 25362.0, 0.7183),
        Body::new("Neptune", 6.835107e6, 24624.0, 0.67125),
        Body::new("Pluto", 872.4, 1195.0, 5.342128),
    ];

    // Adding some padding around the table
    println!();

    for planet in planets.iter() {
        let surf_grav = format!("{:e}", planet.surface_gravity());
        let surf_grav_rot = format!("{:e}", planet.sg_with_rotation());
        data_table.add_row(row![planet.name(), surf_grav, surf_grav_rot]);
    }
    data_table.printstd();

    println!("\n1st Frame\n---------");

    /* I normalized these on initialization since
     * the full vector is not needed
     */
    let mercury = Vector3::new(
        1.337560590950003e7,
        -6.546401542548555e7,
        -6.688126999248266e6,
    )
    .normalize();

    let jupiter = Vector3::new(
        -2.988516576699104e8,
        -7.403687698930221e8,
        9.755579094294041e6,
    )
    .normalize();
    let earth = Vector3::new(
        -7.457928304815032e7,
        1.281079804505313e8,
        -1.353099709045142e4,
    )
    .normalize();

    println!("{} Units: {:#?}", "Mercury", mercury);
    println!("{} {:#?}", "Earth", earth);
    println!("{} {:#?}", "Jupiter", jupiter);
    println!("Angle: {}", get_angle(mercury, jupiter, earth));
}
