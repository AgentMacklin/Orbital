#![allow(dead_code)]
#![allow(unused_doc_comments)]

extern crate nalgebra;
use nalgebra::{Matrix3, Vector3};

const SOLARGM: f64 = 1.328905188132376e11;

/* Orbital bodies, in this case New Horizons and Ultima Thule */
pub struct Body {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
}

/* Adds methods to Body struct */
impl Body {
    pub fn radial_veloc(&self) -> Vector3<f64> {
        (self.velocity.dot(&self.position) / self.position.norm_squared()) * self.position
    }

    pub fn tangential_veloc(&self) -> Vector3<f64> {
        self.omega().cross(&self.position)
    }

    pub fn true_anomaly(&self) -> f64 {
        let e_vec = self.eccentricity_vec();
        let posit = self.position.normalize();
        let val = e_vec.dot(&posit) / (e_vec.norm() * posit.norm());
        val.acos().to_degrees()
    }

    /* points from focus to perigee if I'm not mistaken */
    pub fn eccentricity_vec(&self) -> Vector3<f64> {
        (self.velocity.cross(&self.angular_momentum())) / SOLARGM - self.position.normalize()
    }

    pub fn angular_momentum(&self) -> Vector3<f64> {
        self.position.cross(&self.velocity)
    }

    pub fn total_energy(&self) -> f64 {
        let posit = self.position.norm();
        let veloc = self.velocity.norm();
        0.5 * veloc.powi(2) - (SOLARGM / posit)
    }

    pub fn omega(&self) -> Vector3<f64> {
        self.angular_momentum() / self.position.norm_squared()
    }

    pub fn rotation_rate(&self) -> f64 {
        self.angular_momentum().norm() / self.position.norm_squared()
    }

    // Relative velocity between two bodies
    pub fn relative_velocity(&self, other: &Body) -> Vector3<f64> {
        let total_omega = self.omega() + other.omega();
        self.velocity - total_omega.cross(&self.position)
    }

    // Angle to other body
    pub fn angle_to(&self, other: &Body) -> f64 {
        let coeff = self.position.cross(&other.position).magnitude();
        let dot = self.position.dot(&other.position);
        return coeff.atan2(dot).to_degrees();
    }
}

/* create a transform matrix given an array of unit vectors for two frames */
pub fn transform(frame_one: [Vector3<f64>; 3], frame_two: [Vector3<f64>; 3]) -> Matrix3<f64> {
    Matrix3::new(
        frame_two[0].dot(&frame_one[0]),
        frame_two[1].dot(&frame_one[0]),
        frame_two[2].dot(&frame_one[0]),
        frame_two[0].dot(&frame_one[1]),
        frame_two[1].dot(&frame_one[1]),
        frame_two[2].dot(&frame_one[1]),
        frame_two[0].dot(&frame_one[2]),
        frame_two[1].dot(&frame_one[2]),
        frame_two[2].dot(&frame_one[2]),
    )
}

/* Macro that makes it more convenient to print out results */
macro_rules! printer {
    // print vector
    ($msg:expr, v => $val:expr) => {
        let units = ["x", "y", "z"];
        println!("{}:", $msg.cyan());
        for (index, element) in $val.iter().enumerate() {
            println!("  {}: {:.10e}", units[index].green(), element);
        }
        println!();
    };
    // print scalar in scientific notation (s => denotes $val is a scalar and not a vector)
    ($msg:expr, s => $val:expr) => {
        println!("{}:\n  {:.10e}\n", $msg.green(), $val)
    };

    // print matrix
    ($msg:expr, m => $val:expr) => {
        println!("{}:", $msg.cyan());
        for i in 0..3 {
            let row = $val.row(i);
            println!("{:>13.6e} {:>13.6e} {:>13.6e}", row[0], row[1], row[2]);
        }
        println!();
    };
}
