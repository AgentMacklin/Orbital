#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * body.rs contains the Body struct and implements methods for it. A body struct contains only the
 * position and velocity vectors of the body, other parameters are calculated using methods. A body
 * is instantiated using using the Body::new() method, which also determines the type of orbit the
 * body has at the same time. orbit_type does not have to be given manually.
 */
use nalgebra::{Matrix3, Vector3};
use std::f64::consts::PI;

use colored::*;
const DAYTOSEC: f64 = 24.0 * 3600.0;
const SOLARGM: f64 = 2.963092749241593e-4;

const PI2: f64 = 2.0 * PI;

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
    pub orbit_type: OrbitType,
}

/* Adds methods to Body struct */
impl Body {
    pub fn new(position: Vector3<f64>, velocity: Vector3<f64>) -> Body {
        // h and e are used for determining what kind of orbit the body is currently in
        let h = position.cross(&velocity);
        let e = ((velocity.cross(&h) / SOLARGM) - position.normalize()).norm();
        Body {
            position: position,
            velocity: velocity,
            orbit_type: OrbitType::new(e),
        }
    }

    pub fn radial_velocity(&self) -> Vector3<f64> {
        (self.velocity.dot(&self.position) / self.position.norm_squared()) * self.position
    }

    pub fn tangential_velocity(&self) -> Vector3<f64> {
        self.omega().cross(&self.position)
    }

    pub fn true_anomaly(&self) -> f64 {
        let e_vec = self.eccentricity_vector();
        let posit = self.position.normalize();
        let val = e_vec.dot(&posit) / (e_vec.norm() * posit.norm());
        if posit.dot(&self.velocity.normalize()) < 0.0 {
            return PI2 - val.acos();
        } else {
            return val.acos();
        }
    }

    /// Position vector at a time in the future, starting from now
    pub fn position_at_time(&self, time: f64) -> Vector3<f64> {
        let t_anom = self.true_anomaly_at_time(time);
        let omega = self.argument_of_periapsis() - t_anom;
        let inc = self.inclination();
        let tht = self.argument_of_ascending_node();
        let trans_mat = three_one_three_transform(omega, inc, tht)
            .try_inverse()
            .unwrap();
        let p = self.position_at_angle(t_anom);
        return trans_mat * p;
    }

    /// Position vector at a time in the future, starting from now
    pub fn velocity_at_time(&self, time: f64) -> Vector3<f64> {
        let t_anom = self.true_anomaly_at_time(time);
        let omega = self.argument_of_periapsis() - t_anom;
        let inc = self.inclination();
        let tht = self.argument_of_ascending_node();
        let trans_mat = three_one_three_transform(omega, inc, tht)
            .try_inverse()
            .unwrap();
        let v = self.velocity_at_angle(t_anom);
        return trans_mat * v;
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
        let p = self.orbital_parameter();
        let radius = p / (1.0 + e * angle.cos());
        Vector3::new(radius, 0.0, 0.0)
    }

    pub fn velocity_at_angle(&self, angle: f64) -> Vector3<f64> {
        let coeff = SOLARGM / self.angular_momentum().norm();
        let e = self.eccentricity();
        Vector3::new(coeff * -angle.sin(), coeff * (e + angle.cos()), 0.0)
    }

    // Angle to other body, keep getting the wrong thing anyway, tried everything
    pub fn angle_to(&self, other: &Body) -> f64 {
        (self.position.dot(&other.position) / (self.position.norm() * other.position.norm())).acos()
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
        2.0 * ((theta / 2.0).tan() / ((1.0 + e) / (1.0 - e)).sqrt()).atan()
    }

    pub fn time_since_periapsis(&self) -> f64 {
        let t_anom = self.true_anomaly();
        let e_anom = self.true_to_eccentric(t_anom);
        let a = self.semi_major_axis();
        let e = self.eccentricity();
        (a.powi(3) / SOLARGM).sqrt() * (e_anom - e * e_anom.sin())
    }

    pub fn eccentricity(&self) -> f64 {
        self.eccentricity_vector().norm()
    }

    pub fn inclination(&self) -> f64 {
        let h = self.angular_momentum();
        (h[2] / h.norm()).acos() // h[2] is the z component of the vector
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
            PI2 - omega
        } else {
            omega
        }
    }

    pub fn argument_of_ascending_node(&self) -> f64 {
        let n = self.ascending_node();
        let n_x = n[0];
        let n_y = n[1];
        if n_y >= 0.0 {
            (n_x / n.norm()).acos()
        } else {
            PI2 - (n_x / n.norm()).acos()
        }
    }

    pub fn true_to_eccentric(&self, t_anom: f64) -> f64 {
        let a = self.semi_major_axis();
        let e = self.eccentricity();
        let b = a * (1.0 - e.powi(2)).sqrt();
        let p = self.orbital_parameter();
        let r = p / (1.0 + e * t_anom.cos());
        let c = (a * e + r * t_anom.cos()) / a;
        let s = (r / b) * t_anom.sin();
        return s.atan2(c);
    }

    pub fn true_anomaly_at_time(&self, time: f64) -> f64 {
        let t_peri = self.time_since_periapsis();
        let m_anom = self.mean_anomaly(time + t_peri);
        let angle = self.eccentric_from_mean(m_anom);
        return PI2 - self.eccentric_to_true_anomaly(angle);
    }

    /// The eccentric anomaly at a certain time
    pub fn eccentric_from_mean(&self, m_anom: f64) -> f64 {
        match self.kepler(m_anom) {
            Ok(num) => num,
            Err(e) => {
                eprintln!("{}: {}\n", "Invalid Orbit".red(), e);
                return std::f64::NAN;
            }
        }
    }

    /// Return the eccentric anomaly using the appropriate Kepler equation
    pub fn kepler(&self, m_anom: f64) -> Result<f64, &str> {
        let e = self.eccentricity();
        match &self.orbit_type {
            OrbitType::Elliptic => Ok(elliptic_kepler(m_anom, e)),
            OrbitType::Hyperbolic => Ok(hyper_kepler(m_anom, e)),
            OrbitType::Circular => Err("cannot use Keler's equation with a circular orbit."),
            OrbitType::Parabolic => Err("cannot use Kepler's equation with a parabolic orbit."),
        }
    }

    pub fn eccentric_to_true_anomaly(&self, e_anom: f64) -> f64 {
        let e = self.eccentricity();
        // let sqrt_val = ((1.0 + e) / (1.0 - e)).sqrt();
        // 2.0 * (sqrt_val * (e_anom / 2.0).tan()).atan() + PI2
        ((e_anom.cos() - e) / (1.0 - e * e_anom.cos())).acos()
    }

    /// Return the mean anomaly at a certain time from current position
    pub fn mean_anomaly(&self, t: f64) -> f64 {
        let n = (SOLARGM / self.semi_major_axis().powi(3)).sqrt();
        n * t
    }
}

/**
 * Some of the kepler functions below. Body matches on its orbit type
 * and uses the correct function to return the correct eccentric anomaly
 */
fn elliptic_kepler(nt: f64, eccen: f64) -> f64 {
    let tolerance = 1e-15;
    let kep = |e: f64| e - eccen * e.sin() - nt;
    let kep_d = |e: f64| 1.0 - eccen * e.cos();
    let mut e_0 = 0.0;
    let mut e = e_0 - (kep(e_0) / kep_d(e_0));
    while (e - e_0).abs() > tolerance {
        e_0 = e;
        e = e_0 - (kep(e_0) / kep_d(e_0));
    }
    return e;
}

fn hyper_kepler(nt: f64, eccen: f64) -> f64 {
    let tolerance = 1e-15;
    let kep = |e: f64| eccen * e.sinh() - nt - e;
    let kep_d = |e: f64| eccen * e.cosh() - 1.0;
    let mut e_0 = nt;
    let mut e = e_0 - kep(e_0) / kep_d(e_0);
    while (e - e_0).abs() > tolerance {
        e_0 = e;
        e = e_0 - kep(e_0) / kep_d(e_0);
    }
    return e;
}

pub fn three_one_three_transform(
    arg_of_peri: f64,
    inclination: f64,
    arg_of_an: f64,
) -> Matrix3<f64> {
    let omega = arg_of_peri;
    let inc = inclination;
    let tht = arg_of_an;
    let m_c = Matrix3::new(
        omega.cos(),
        omega.sin(),
        0.0,
        -omega.sin(),
        omega.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    );
    let m_b = Matrix3::new(
        1.0,
        0.0,
        0.0,
        0.0,
        inc.cos(),
        inc.sin(),
        0.0,
        -inc.sin(),
        inc.cos(),
    );
    let m_a = Matrix3::new(
        tht.cos(),
        tht.sin(),
        0.0,
        -tht.sin(),
        tht.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    );

    return m_c * m_b * m_a;
}
