/**
 * Implementation of Lambert algorithm
 */

use nalgebra::Vector3;

use super::body::Body;

pub fn lambert(body: &Body) -> Vector3<f64> {
    let t_frame = body.make_frame();
    return t_frame * body.angular_momentum();
}