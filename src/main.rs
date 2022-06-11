mod helpers;
mod models;

use std::time::Instant;

use crate::models::{matrices::Matrix, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let matrix = Matrix::square(
        4,
        Vec::from([
            Vec::from([-2., -8., 3., 5.]),
            Vec::from([-3., 1., 7., 3.]),
            Vec::from([1., 2., -9., 6.]),
            Vec::from([-6., 7., 7., -9.]),
        ]),
    );

    println!("{}", matrix);
    println!("{}", matrix.det());

    println!("Program took: {:?}", started_at.elapsed());
}
