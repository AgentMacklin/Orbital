/**
 * A body is any orbital entity, and has gm, radius, and
 * an Orbit struct which holds a bunch of its orbital parameters
 */
use std::f64::consts::PI;
// const GRAV_CONST: f64 = 6.67384e-11;

/**
 * A body describes an orbitable body with a name (for printing), GM, and radius.
 * It also has other optional parameters that can be specified according to the
 * body's use case (like calculating the centripetal acceleration using the body's
 * length of day, which is not needed for other calculations).
 */
#[derive(Debug, Clone)]
pub struct Body {
    pub gm: f64,
    pub radius: f64,
    pub name: Option<String>,
    pub day_len: Option<f64>,
}

impl Body {
    // Create a body with the length of its day
    pub fn new(name: &str, gm: f64, radius: f64, day_len: f64) -> Body {
        Body {
            gm: gm,
            radius: radius,
            name: Some(String::from(name)),
            day_len: Some(day_len),
        }
    }

    // Return surface gravity of Body
    pub fn surface_gravity(&self) -> f64 {
        // 1000.0 converts km to m
        1000.0 * self.gm / (self.radius * self.radius)
    }

    pub fn name(&self) -> String {
        match self.name.as_ref() {
            Some(name) => name.clone(),
            None => panic!("Body doesn't have a name."),
        }
    }

    // surface gravity with rotation
    pub fn sg_with_rotation(&self) -> f64 {
        let rot_veloc = self.rotational_velocity();
        let grav_accel = self.surface_gravity();
        grav_accel - (1000.0 * (rot_veloc * rot_veloc) / self.radius)
    }

    // Return the tangential velocity at the equator of a body
    fn rotational_velocity(&self) -> f64 {
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
