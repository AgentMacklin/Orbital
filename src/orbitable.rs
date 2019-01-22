/**
 * A body is any orbital entity, and has gm, radius, and
 * an Orbit struct which holds a bunch of its orbital parameters
 */
use std::f64::consts::PI;
// const GRAV_CONST: f64 = 6.67384e-11;

// Defines orbital parameters for a body
#[derive(Debug)]
pub struct Orbit {
    pub inclination: f64,
    pub eccentricity: f64,
    pub semi_major: f64,
}

/**
 * A body with an optional orbit argument, which describes the body's orbital
 * parameters if need be. For example, if you were going to calculate the gravity
 * at the surface of the Body, you don't need the Body's orbital parameters for
 * that, so you can pass in None for orbit and skip that altogether
 */
#[derive(Debug)]
pub struct Body {
    pub gm: f64,
    pub radius: f64,
    pub day_len: Option<f64>,
    pub orbit: Option<Orbit>,
}

impl Orbit {
    pub fn new(incl: f64, eccen: f64, semi_major: f64) -> Orbit {
        Orbit {
            inclination: incl,
            eccentricity: eccen,
            semi_major: semi_major,
        }
    }
}

impl Body {
    // Create a new Body with optional orbit parameters
    pub fn new(gm: f64, radius: f64) -> Body {
        Body {
            gm: gm,
            radius: radius,
            day_len: None,
            orbit: None,
        }
    }
    // Create a body with the length of its day
    pub fn with_day(gm: f64, radius: f64, day_len: f64) -> Body {
        Body {
            gm: gm,
            radius: radius,
            day_len: Some(day_len),
            orbit: None,
        }
    }

    // Return surface gravity of Body
    pub fn surface_gravity(&self) -> f64 {
        // 1000.0 converts km to m
        1000.0 * self.gm / (self.radius * self.radius)
    }

    // surface gravity with rotation
    pub fn sg_with_rotation(&self) -> f64 {
        let rot_veloc = self.rotational_veloc();
        let grav_accel = self.surface_gravity();
        grav_accel - (1000.0 * (rot_veloc * rot_veloc) / self.radius)
    }

    // Return the tangential velocity at the equator of a body
    fn rotational_veloc(&self) -> f64 {
        // 1 day of body in seconds
        (2.0 * PI * self.radius) / day_to_seconds(self.day_len)
    }
}

fn day_to_seconds(day_len: Option<f64>) -> f64 {
    match day_len {
        Some(num) => num.abs() * 24.0 * 3600.0,
        None => panic!("A length of day has not been specified for body."),
    }
}
