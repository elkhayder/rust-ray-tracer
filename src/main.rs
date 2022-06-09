mod helpers;
mod models;

use std::time::Instant;

use crate::models::{matrices::Matrix, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let matrix4_1 = Matrix::square(
        4,
        Vec::from([
            Vec::from([1.0, 2.0, 3.0, 4.0]),
            Vec::from([5.0, 6.0, 7.0, 8.0]),
            Vec::from([9.0, 8.0, 7.0, 6.0]),
            Vec::from([5.0, 4.0, 3.0, 2.0]),
        ]),
    );
    let matrix4_2 = Matrix::square(
        4,
        Vec::from([
            Vec::from([-2.0, 1.0, 2.0, 3.0]),
            Vec::from([3.0, 2.0, 1.0, -1.0]),
            Vec::from([4.0, 3.0, 6.0, 5.0]),
            Vec::from([1.0, 2.0, 7.0, 8.0]),
        ]),
    );

    println!("{}", matrix4_1);
    println!("{}", matrix4_2);

    println!("{}", &matrix4_1 * &matrix4_2);
    println!(
        "{}",
        Tuple::from(&Matrix {
            rows: 4,
            columns: 1,
            data: vec![vec![0.0]; 4]
        })
    );

    println!("{}", Matrix::from(&Tuple::vector(1.0, 2.0, 3.0)));

    println!("Program took: {:?}", started_at.elapsed());
}
