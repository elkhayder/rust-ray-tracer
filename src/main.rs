mod helpers;
mod models;

use std::{f64::consts::PI, time::Instant};

use crate::models::{axis::Axis, matrices::Matrices, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let point = Tuple::point(2.0, 3.0, 4.0);

    println!(
        "{}",
        &Matrices::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * &point
    );

    println!("Program took: {:?}", started_at.elapsed());
}
