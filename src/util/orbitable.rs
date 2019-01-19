/**
 * A body is any orbital entity, and has mass, radius, and
 * an Orbit struct which holds a bunch of its orbital parameters
 * * Only does basic stuff for now
 */
use std::f64::consts::PI;
const GRAV_CONST: f64 = 6.67384e-11;

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
    pub mass: f64,
    pub radius: f64,
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
    pub fn new(mass: f64, radius: f64, orbit: Option<Orbit>) -> Body {
        Body {
            mass: mass,
            radius: radius,
            orbit: orbit,
        }
    }
    // return the average density of the Body
    pub fn density(&self) -> f64 {
        let volume: f64 = (4.0 / 3.0) * PI * self.radius;
        self.mass / volume
    }

    // Return surface gravity of Body
    pub fn surface_gravity(&self) -> f64 {
        (GRAV_CONST * self.mass) / (self.radius.powi(2))
    }
}
