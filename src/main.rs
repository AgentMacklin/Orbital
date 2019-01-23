#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

mod orbitable;
use self::orbitable::Body;
<<<<<<< HEAD
=======
use std::collections::HashMap;
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330

fn main() {
    // Pretty print data to a table so it's easier to read
    let mut data_table = Table::new();
    data_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    data_table.add_row(row![
        "Planet",
        "Surface Gravity",
        "Surface Gravity w/ Rotation"
    ]);

<<<<<<< HEAD
    // An array of planets
    let planets = vec![
        Body::with_day("Mercury", 22032.09, 2440.0, 58.6462),
        Body::with_day("Venus", 324858.63, 6051.8, -243.0185),
        Body::with_day("Earth", 398600.440, 6371.01, 0.997257916),
        Body::with_day("Mars", 42828.3, 3389.9, 1.0274907),
        Body::with_day("Vesta", 17.8, 262.7, 0.2225886),
        Body::with_day("Ceres", 62.6284, 470.0, 0.37809041),
        Body::with_day("Jupiter", 1.26686511e8, 69911.0, 0.413538),
        Body::with_day("Saturn", 3.79312078e7, 58232.0, 0.4440083),
        Body::with_day("Uranus", 5.793966e6, 25362.0, 0.7183),
        Body::with_day("Neptune", 6.835107e6, 24624.0, 0.67125),
        Body::with_day("Pluto", 872.4, 1195.0, 5.342128),
    ];

    // Adding some padding around the table
    println!();

    for planet in planets.iter() {
        let surf_grav = format!("{:e}", planet.surface_gravity());
        let surf_grav_rot = format!("{:e}", planet.sg_with_rotation());
        data_table.add_row(row![planet.name, surf_grav, surf_grav_rot]);
=======
    // A very messy dictionary of planets
    let mut planets: HashMap<&str, Body> = HashMap::new();
    planets.insert("Mercury", Body::with_day(22032.09, 2440.0, 58.6462));
    planets.insert("Venus", Body::with_day(324858.63, 6051.8, -243.0185));
    planets.insert("Earth", Body::with_day(398600.440, 6371.01, 0.997257916));
    planets.insert("Mars", Body::with_day(42828.3, 3389.9, 1.0274907));
    planets.insert("Vesta", Body::with_day(17.8, 262.7, 0.2225886));
    planets.insert("Ceres", Body::with_day(62.6284, 470.0, 0.37809041));
    planets.insert("Jupiter", Body::with_day(1.26686511e8, 69911.0, 0.413538));
    planets.insert("Saturn", Body::with_day(3.79312078e7, 58232.0, 0.4440083));
    planets.insert("Uranus", Body::with_day(5.793966e6, 25362.0, 0.7183));
    planets.insert("Neptune", Body::with_day(6.835107e6, 24624.0, 0.67125));
    planets.insert("Pluto", Body::with_day(872.4, 1195.0, 5.342128));

    // Adding some padding to the table
    println!();

    // Hashmaps don't iterate in order so the order might be weird
    for (key, value) in planets.iter() {
        let surf_grav = format!("{:e}", value.surface_gravity());
        let surf_grav_rot = format!("{:e}", value.sg_with_rotation());
        data_table.add_row(row![key, surf_grav, surf_grav_rot]);
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
    }
    data_table.printstd();
    println!();
}
