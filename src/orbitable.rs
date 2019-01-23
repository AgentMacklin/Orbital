/**
 * A body is any orbital entity, and has gm, radius, and
 * an Orbit struct which holds a bunch of its orbital parameters
 */
use std::f64::consts::PI;
// const GRAV_CONST: f64 = 6.67384e-11;

<<<<<<< HEAD
=======
// Defines orbital parameters for a body
#[derive(Debug)]
pub struct Orbit {
    pub inclination: f64,
    pub eccentricity: f64,
    pub semi_major: f64,
}

>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
/**
 * A body with an optional orbit argument, which describes the body's orbital
 * parameters if need be. For example, if you were going to calculate the gravity
 * at the surface of the Body, you don't need the Body's orbital parameters for
 * that, so you can pass in None for orbit and skip that altogether
 */
#[derive(Debug)]
pub struct Body {
<<<<<<< HEAD
    pub name: String,
=======
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
    pub gm: f64,
    pub radius: f64,
    pub day_len: Option<f64>,
    pub orbit: Option<Orbit>,
<<<<<<< HEAD
    pub position: Option<Vec3>,
}

// Defines orbital parameters for a body
#[derive(Debug)]
pub struct Orbit {
    pub inclination: f64,
    pub eccentricity: f64,
    pub semi_major: f64,
}

// A 3-dimensional vector
#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
=======
}

impl Orbit {
    pub fn new(incl: f64, eccen: f64, semi_major: f64) -> Orbit {
        Orbit {
            inclination: incl,
            eccentricity: eccen,
            semi_major: semi_major,
        }
    }
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
}

impl Body {
    // Create a new Body with optional orbit parameters
<<<<<<< HEAD
    pub fn default(name: &str, gm: f64, radius: f64) -> Body {
        Body {
            name: String::from(name),
=======
    pub fn new(gm: f64, radius: f64) -> Body {
        Body {
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
            gm: gm,
            radius: radius,
            day_len: None,
            orbit: None,
<<<<<<< HEAD
            position: None,
        }
    }
    // Create a body with the length of its day
    pub fn with_day(name: &str, gm: f64, radius: f64, day_len: f64) -> Body {
        Body {
            name: String::from(name),
=======
        }
    }
    // Create a body with the length of its day
    pub fn with_day(gm: f64, radius: f64, day_len: f64) -> Body {
        Body {
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
            gm: gm,
            radius: radius,
            day_len: Some(day_len),
            orbit: None,
<<<<<<< HEAD
            position: None,
=======
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
        }
    }

    // Return surface gravity of Body
    pub fn surface_gravity(&self) -> f64 {
        // 1000.0 converts km to m
        1000.0 * self.gm / (self.radius * self.radius)
    }

    // surface gravity with rotation
    pub fn sg_with_rotation(&self) -> f64 {
<<<<<<< HEAD
        let rot_veloc = self.rotational_velocity();
=======
        let rot_veloc = self.rotational_veloc();
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
        let grav_accel = self.surface_gravity();
        grav_accel - (1000.0 * (rot_veloc * rot_veloc) / self.radius)
    }

    // Return the tangential velocity at the equator of a body
<<<<<<< HEAD
    fn rotational_velocity(&self) -> f64 {
=======
    fn rotational_veloc(&self) -> f64 {
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
        // 1 day of body in seconds
        (2.0 * PI * self.radius) / day_to_seconds(self.day_len)
    }
}

<<<<<<< HEAD
impl Orbit {
    pub fn new(incl: f64, eccen: f64, semi_major: f64) -> Orbit {
        Orbit {
            inclination: incl,
            eccentricity: eccen,
            semi_major: semi_major,
        }
    }
}

impl Vec3 {
    // New Vec3
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    // Magnitude of vector
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    // return a Vec3 which is a normalized version of previous Vec3
    pub fn normalized(&self) -> Vec3 {
        let mag = self.magnitude();
        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

=======
>>>>>>> d4e0cb4d39eed4456062776765052dbc0a668330
fn day_to_seconds(day_len: Option<f64>) -> f64 {
    match day_len {
        Some(num) => num.abs() * 24.0 * 3600.0,
        None => panic!("A length of day has not been specified for body."),
    }
}
