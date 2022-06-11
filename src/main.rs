mod helpers;
mod models;

use std::time::Instant;

use crate::models::{matrices::Matrix, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let matrix = Matrix::square(
        4,
        Vec::from([
            Vec::from([8., -5., 9., 2.]),
            Vec::from([7., 5., 6., 1.]),
            Vec::from([-6., 0., 9., 6.]),
            Vec::from([-3., 0., -9., -4.]),
        ]),
    );

    println!("{}", matrix.inverse());

    println!("Program took: {:?}", started_at.elapsed());
}
