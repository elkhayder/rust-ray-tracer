mod helpers;
mod models;

use std::{f64::consts::PI, time::Instant};

use crate::models::{axis::Axis, matrices::Matrices, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let point = Tuple::point(2.0, 3.0, 4.0);

    println!(
        "{}",
        point.apply(&[
            &Matrices::rotation(Axis::X, PI),
            &Matrices::scaling(2., 3., -2.),
            &Matrices::translation(10., -2., 5.54)
        ])
    );

    println!("Program took: {:?}", started_at.elapsed());
}
