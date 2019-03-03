/**
 * Return the angle between vectors
 */
extern crate nalgebra as na;
use na::Vector3;

// |A.B| = |A| |B| COS()
// |AxB| = |A| |B| SIN()
pub fn get_angle(mercury: Vector3<f64>, jupiter: Vector3<f64>, earth: Vector3<f64>) -> f64 {
    let earth_to_mercury = mercury - earth;
    let earth_to_jupiter = jupiter - earth;
    let coeff = earth_to_mercury.cross(&earth_to_jupiter).magnitude();
    let dot = earth_to_mercury.dot(&earth_to_jupiter);
    return coeff.atan2(dot).to_degrees();
}
