#![allow(dead_code)]
#![allow(unused_doc_comments)]

extern crate nalgebra;
use nalgebra::{Matrix3, Vector3};
use std::f64::consts::PI;

const SOLARGM: f64 = 1.328905188132376e11;

/**
 * OrbitType is used to abstract away some of the functions that depend on
 * the type of orbit the body is in, like kepler's equation. That way, you
 * can call one function and it will return the correct value
 */
#[derive(Debug)]
pub enum OrbitType {
    Circular,
    Elliptic,
    Parabolic,
    Hyperbolic,
}

impl OrbitType {
    /// Return the orbit type given the eccentricity, Body::new
    /// uses this function to set the orbit type when an instance
    /// is constructed
    pub fn new(eccentricity: f64) -> OrbitType {
        if eccentricity == 0.0 {
            return OrbitType::Circular;
        } else if eccentricity < 1.0 && eccentricity > 0.0 {
            return OrbitType::Elliptic;
        } else if eccentricity == 1.0 {
            return OrbitType::Parabolic;
        } else {
            return OrbitType::Hyperbolic;
        }
    }
}

#[derive(Debug)]
pub struct Body {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub orbitType: OrbitType,
}

/* Adds methods to Body struct */
impl Body {
    pub fn new(position: Vector3<f64>, velocity: Vector3<f64>) -> Body {
        // Used for determining what kind of orbit the body is currently in
        let h = position.cross(&velocity);
        let e = ((velocity.cross(&h) / SOLARGM) - position.normalize()).norm();
        Body {
            position: position,
            velocity: velocity,
            orbitType: OrbitType::new(e),
        }
    }

    pub fn radial_veloc(&self) -> Vector3<f64> {
        (self.velocity.dot(&self.position) / self.position.norm_squared()) * self.position
    }

    pub fn tangential_veloc(&self) -> Vector3<f64> {
        self.omega().cross(&self.position)
    }

    pub fn true_anomaly(&self) -> f64 {
        let e_vec = self.eccentricity_vector();
        let posit = self.position.normalize();
        let val = e_vec.dot(&posit) / (e_vec.norm() * posit.norm());
        if posit.dot(&self.velocity.normalize()) < 0.0 {
            return (2.0 * PI - val.acos()).to_degrees();
        } else {
            return val.acos().to_degrees();
        }
    }

    /* points from focus to perigee if I'm not mistaken */
    pub fn eccentricity_vector(&self) -> Vector3<f64> {
        let veloc = self.velocity;
        let posit = self.position;
        let h = self.angular_momentum();
        (veloc.cross(&h) / SOLARGM) - posit.normalize()
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
        let e = self.eccentricity();
        let numer = self.angular_momentum().norm_squared() / SOLARGM;
        let denom = 1_f64 + (e * (angle.to_radians()).cos());
        let radius = numer / denom;
        Vector3::new(radius, 0.0, 0.0)
    }

    pub fn velocity_at_angle(&self, angle: f64) -> Vector3<f64> {
        let p = self.orbital_parameter();
        let e = self.eccentricity();
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
        let e = self.eccentricity();
        ang_moment.powi(2) / (SOLARGM * (1_f64 - e.powi(2)))
    }

    pub fn orbital_period(&self) -> f64 {
        2_f64 * PI * (self.semi_major_axis().powi(3) / SOLARGM).sqrt()
    }

    pub fn orbital_parameter(&self) -> f64 {
        let e = self.eccentricity();
        self.semi_major_axis() * (1.0 - e.powi(2))
    }

    pub fn eccentric_anomaly(&self) -> f64 {
        let e = self.eccentricity();
        let theta = self.true_anomaly();
        2.0 * ((theta.to_radians() / 2.0).tan() / ((1.0 + e) / (1.0 - e)).sqrt())
            .atan()
            .to_degrees()
            + 360.0
    }

    pub fn time_since_periapsis(&self) -> f64 {
        let E = self.eccentric_anomaly().to_radians();
        let a = self.semi_major_axis();
        let e = self.eccentricity();
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
        let eccen = self.eccentricity();
        elliptic_kepler_iterate(0.0, nt, eccen).to_degrees()
    }

    pub fn eccen_to_true_anomaly(&self, eccen_anom: f64) -> f64 {
        let e = self.eccentricity();
        2.0 * (((1.0 + e) / (1.0 - e)).sqrt() * (eccen_anom / 2.0).tan())
            .atan()
            .to_degrees()
    }

    pub fn eccentricity(&self) -> f64 {
        self.eccentricity_vector().norm()
    }

    pub fn inclination(&self) -> f64 {
        let h = self.angular_momentum();
        // h[2] is the z component of the vector
        (h[2] / h.norm()).acos().to_degrees()
    }

    pub fn ascending_node(&self) -> Vector3<f64> {
        let k = Vector3::new(0.0, 0.0, 1.0);
        k.cross(&self.angular_momentum())
    }

    pub fn argument_of_periapsis(&self) -> f64 {
        let n = self.ascending_node();
        let e = self.eccentricity_vector();
        let omega = (n.dot(&e) / (n.norm() * e.norm())).acos();
        if e[2] < 0.0 {
            (2.0 * PI - omega).to_degrees()
        } else {
            omega.to_degrees()
        }
    }

    pub fn argument_of_ascending_node(&self) -> f64 {
        let n = self.ascending_node();
        let n_x = n[0];
        let n_y = n[1];
        if n_y >= 0.0 {
            (n_x / n.norm()).acos().to_degrees()
        } else {
            (2.0 * PI - (n_x / n.norm()).acos()).to_degrees()
        }
    }

    pub fn kepler(&self, init: f64, time: f64) -> Result<f64, &str> {
        let mean_anom = self.mean_anomaly(time);
        let e = self.eccentricity();
        match &self.orbitType {
            Elliptic => Ok(elliptic_kepler_iterate(init, mean_anom, e)),
            Hyperbolic => Ok(hyper_kepler_iterate(init, mean_anom, e)),
            // Technically you could use other equations for these,
            // but returning errors for now
            Circular => Err("Cannot use Kepler's equation with a circular orbit."),
            Parabolic => Err("Cannot use Kepler's equation with a parabolic orbit."),
        }
    }
}

/**
 * Some of the kepler functions below. Body matches on its orbit type
 * and uses the correct function to return the correct eccentric anomaly
 */
fn elliptic_kepler(e: f64, nt: f64, eccen: f64) -> f64 {
    let delta_e = (e - eccen * e.sin() - nt) / (1.0 - eccen * e.cos());
    return e - delta_e;
}

fn elliptic_kepler_iterate(init: f64, nt: f64, eccen: f64) -> f64 {
    let mut e_0 = init;
    let mut e = elliptic_kepler(e_0, nt, eccen);

    while (e - e_0).abs() > 1e-12 {
        e_0 = e;
        e = elliptic_kepler(e_0, nt, eccen);
    }

    return e;
}

fn hyper_kepler(e: f64, nt: f64, eccen: f64) -> f64 {
    let delta_e = (eccen * e.sinh() - nt - e) / (eccen * e.cosh() - 1.0);
    return e - delta_e;
}

fn hyper_kepler_iterate(init: f64, nt: f64, eccen: f64) -> f64 {
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

