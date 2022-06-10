mod helpers;
mod models;

use std::time::Instant;

use crate::models::{matrices::Matrix, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let matrix4_1 = Matrix {
        rows: 2,
        columns: 4,
        data: Vec::from([
            Vec::from([1.0, 2.0, 3.0, 2.0]),
            Vec::from([10.0, 21.0, 32.0, 43.0]),
        ]),
    };

    let matrix4_2 = Matrix {
        rows: 4,
        columns: 3,
        data: Vec::from([
            Vec::from([5.0, 10.0, 15.0]),
            Vec::from([2.0, 4.0, 6.0]),
            Vec::from([9.0, 9.0, 1.0]),
            Vec::from([1.0, 2.0, 3.0]),
        ]),
    };

    let mul = &matrix4_1 * &matrix4_2;

    println!("{}", matrix4_1);
    println!("{}", matrix4_2);
    println!("{}", mul);

    // let x = &matrix4 * &tuple;

    // println!("matrix4 * tuple = {}", x);

    println!("Program took: {:?}", started_at.elapsed());
}
