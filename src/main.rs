mod helpers;
mod models;

use std::time::Instant;

use crate::models::matrices::Matrix;

fn main() {
    let started_at = Instant::now();

    let mut matrix4_1 = Matrix::new(
        4,
        Vec::from([
            Vec::from([0.0, 0.0, 0.23238222222888222222, 0.0]),
            Vec::from([0.0, 0.0, 0.0, 0.0]),
            Vec::from([0.0, 0.0, 0.0, 0.0]),
            Vec::from([0.0, 0.0, 0.0, 0.0]),
        ]),
    );
    let mut matrix4_2 = Matrix::new(
        4,
        Vec::from([
            Vec::from([0.0, 0.0, 0.23238222222888222223, 0.0]),
            Vec::from([0.0, 0.0, 0.0, 0.0]),
            Vec::from([0.0, 0.0, 0.0, 0.0]),
            Vec::from([0.0, 0.0, 0.0, 0.0]),
        ]),
    );

    // matrix4[1..2] = 4.0;

    let mut matrix2 = Matrix::new(2, Vec::from([Vec::from([0.0, 0.0]), Vec::from([0.0, 0.0])]));

    matrix2[1..1] = 4.0;

    println!("{}", matrix4_1);
    println!("{}", matrix2);

    println!("{}", &matrix4_1 == &matrix4_2);

    println!("Program took: {:?}", started_at.elapsed());
}
