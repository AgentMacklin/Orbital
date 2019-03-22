#![allow(dead_code)]
#![allow(unused_doc_comments)]

extern crate nalgebra;
use nalgebra::{Matrix3, Vector3};
use std::f64::consts::PI;

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
        if posit.dot(&self.velocity.normalize()) < 0.0 {
            return (2.0 * PI - val.acos()).to_degrees();
        } else {
            return val.acos().to_degrees();
        }
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

    pub fn frame_rotation_rate(&self) -> f64 {
        self.omega().norm()
    }

    pub fn position_at_angle(&self, angle: f64) -> Vector3<f64> {
        let e = self.eccentricity_vec().norm();
        let numer = self.angular_momentum().norm_squared() / SOLARGM;
        let denom = 1_f64 + (e * (angle.to_radians()).cos());
        let radius = numer / denom;
        Vector3::new(radius, 0.0, 0.0)
    }

    pub fn velocity_at_angle(&self, angle: f64) -> Vector3<f64> {
        let p = self.orbital_parameter();
        let e = self.eccentricity_vec().norm();
        let h = self.angular_momentum().norm_squared();
        Vector3::new(
            (h / p) * e * angle.to_radians().sin(),
            (h / p) * (1_f64 + e * angle.to_radians().cos()),
            0.0,
        )
    }

    pub fn position_and_velocity(&self, angle: f64) -> (Vector3<f64>, Vector3<f64>) {
        let r = self.position_at_angle(angle);
        let v = self.velocity_at_angle(angle);
        let tht = (angle - self.true_anomaly()).to_radians();
        let trans = Matrix3::from_rows(&[
            Vector3::new(tht.cos(), -tht.sin(), 0.0).transpose(),
            Vector3::new(tht.sin(), tht.cos(), 0.0).transpose(),
            Vector3::new(0.0, 0.0, 1.0).transpose(),
        ]);
        (trans * r, trans * v)
    }

    // Angle to other body, keep getting the wrong thing anyway, tried everything
    pub fn angle_to(&self, other: &Body) -> f64 {
        (self.position.dot(&other.position) / (self.position.norm() * other.position.norm()))
            .acos()
            .to_degrees()
    }

    /* Return a transformation matrix constructed from body's orbit in inertial frame */
    pub fn make_frame(&self) -> Matrix3<f64> {
        let e_r = self.position.normalize();
        let e_h = self.angular_momentum().normalize();
        let e_tht = e_h.cross(&e_r);
        Matrix3::from_rows(&[e_r.transpose(), e_tht.transpose(), e_h.transpose()])
    }

    pub fn semi_major_axis(&self) -> f64 {
        let ang_moment = self.angular_momentum().norm();
        let e = self.eccentricity_vec().norm();
        ang_moment.powi(2) / (SOLARGM * (1_f64 - e.powi(2)))
    }

    pub fn orbital_period(&self) -> f64 {
        2_f64 * PI * (self.semi_major_axis().powi(3) / SOLARGM).sqrt()
    }

    pub fn orbital_parameter(&self) -> f64 {
        let e = self.eccentricity_vec().norm();
        self.semi_major_axis() * (1.0 - e.powi(2))
    }

    pub fn eccentric_anomaly(&self) -> f64 {
        let e = self.eccentricity_vec().norm();
        let theta = self.true_anomaly();
        2.0 * ((theta.to_radians() / 2.0).tan() / ((1.0 + e) / (1.0 - e)).sqrt())
            .atan()
            .to_degrees()
            + 360.0
    }

    pub fn time_since_periapsis(&self) -> f64 {
        let E = self.eccentric_anomaly().to_radians();
        let a = self.semi_major_axis();
        let e = self.eccentricity_vec().norm();
        (a.powi(3) / SOLARGM).sqrt() * (E - e * E.sin())
    }

    /// Return the mean anomaly at a certain time from current position
    pub fn mean_anomaly(&self, time: f64) -> f64 {
        let n = (2.0 * PI) / self.orbital_period();
        n * (time + self.time_since_periapsis())
    }

    /// The eccentric anomaly at a certain time
    pub fn eccen_anom_at_time(&self, time: f64) -> f64 {
        let nt = (self.time_since_periapsis() + time)
            * (SOLARGM / self.semi_major_axis().powi(3)).sqrt();
        let eccen = self.eccentricity_vec().norm();
        kepler_iterate(0.0, nt, eccen).to_degrees()
    }

    pub fn eccen_to_true_anomaly(&self, eccen_anom: f64) -> f64 {
        let e = self.eccentricity_vec().norm();
        2.0 * (((1.0 + e) / (1.0 - e)).sqrt() * (eccen_anom / 2.0).tan())
            .atan()
            .to_degrees()
    }
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

pub fn delta_e(e: f64, nt: f64, eccen: f64) -> f64 {
    (e - eccen * e.sin() - nt) / (1.0 - eccen * e.cos())
}

pub fn kepler(init: f64, nt: f64, eccen: f64) -> f64 {
    init - delta_e(init, nt, eccen)
}

pub fn kepler_iterate(init: f64, nt: f64, eccen: f64) -> f64 {
    let mut e_0 = init;
    let mut e = kepler(e_0, nt, eccen);

    while (e - e_0).abs() > 1e-12 {
        e_0 = e;
        e = kepler(e_0, nt, eccen);
    }

    return e;
}

pub fn hyper_delta_e(e: f64, nt: f64, eccen: f64) -> f64 {
    let numer = eccen * e.sinh() - nt - e;
    let denom = eccen * e.cosh() - 1.0;
    let val = numer / denom;
    return val;
}

pub fn hyper_kepler(init: f64, nt: f64, eccen: f64) -> f64 {
    let val = init - hyper_delta_e(init, nt, eccen);
    return val;
}

pub fn hyper_kepler_iterate(init: f64, nt: f64, eccen: f64) -> f64 {
    let mut e_0 = init;
    let mut e = hyper_kepler(e_0, nt, eccen);

    while (e - e_0).abs() > 1e-12 {
        e_0 = e;
        e = hyper_kepler(e_0, nt, eccen);
        if e == e_0 {
            return e;
        }
    }
    return e;
}
