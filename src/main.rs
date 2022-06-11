mod helpers;
mod models;

use std::{f64::consts::PI, time::Instant};

use float_cmp::approx_eq;

use crate::models::{axis::Axis, matrices::Matrices, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let point = Tuple::point(0., 1., 0.);

    println!("{}", &Matrices::rotation(Axis::Z, PI / 4.) * &point);

    println!("Program took: {:?}", started_at.elapsed());
}
