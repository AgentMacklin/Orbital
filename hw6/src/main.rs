#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 * Exam 3
 */
use nalgebra::Vector3;
const SOLARGM: f64 = 2.963092749241593e-4;

// extern crate colored;
use colored::*;

// fancy progress bars
// use indicatif::{ProgressBar, ProgressStyle};

#[macro_use]
mod macros;
mod body;
mod date;
mod lambert;


use body::Body;

const DAYTOSEC: f64 = 24.0 * 3600.0;

fn main() {
    let mars = Body::new(
        Vector3::new(
            -3.914741740463327E-01,
            -1.436094702371459E+00,
            -2.047823201895446E-02
        ),
        Vector3::new(
            1.401859610775295E-02,
            -2.508557862682466E-03,
            -3.971649629870528E-04
        )
    );
    let t_frame = mars.make_frame();
    
    printer!("Position", v => t_frame * mars.position);
    printer!("Velocity", v => t_frame * mars.velocity);
    printer!("Angular Momentum", v => t_frame * mars.angular_momentum());

}
