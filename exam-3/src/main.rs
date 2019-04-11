#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 */
use nalgebra::Vector3;
const SOLARGM: f64 = 2.963092749241593e-4;

// extern crate colored;
use colored::*;

#[macro_use]
mod macros;
mod body;
mod date;
use body::Body;

const DAYTOSEC: f64 = 24.0 * 3600.0;

fn main() {}
