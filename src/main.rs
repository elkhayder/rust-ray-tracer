mod helpers;
mod models;

use std::time::Instant;

use crate::models::{matrices::Matrix, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let matrix4 = Matrix::square(
        4,
        Vec::from([
            Vec::from([1.0, 2.0, 3.0, 4.0]),
            Vec::from([2.0, 4.0, 4.0, 2.0]),
            Vec::from([8.0, 6.0, 4.0, 1.0]),
            Vec::from([0.0, 0.0, 0.0, 1.0]),
        ]),
    );

    let tuple = Tuple::point(1.0, 2.0, 3.0);

    println!("{}", matrix4);
    println!("{}", Matrix::from(&tuple));

    let x = &matrix4 * &tuple;

    println!("matrix4 * tuple = {}", x);

    println!("Identity 4: {}", Matrix::identity(4));
    println!("Identity 2: {}", Matrix::identity(2));

    println!("Program took: {:?}", started_at.elapsed());
}
