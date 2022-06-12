mod helpers;
mod models;

use std::{f64::consts::PI, time::Instant};

use crate::models::{axis::Axis, canvas::Canvas, colors::Color, matrices::Matrices, tuples::Tuple};

fn main() {
    let started_at = Instant::now();

    let size = 50;

    let mut canvas = Canvas::new(100, 100, None);

    let point = Tuple::point(f64::from(size / 2), f64::from(size / 4), 0.0);

    let mut transformed_point: Tuple;

    let (mut x, mut y): (u32, u32);

    for i in 0..12 {
        transformed_point = &Matrices::rotation(Axis::Z, i as f64 * PI / 6.) * &point;

        x = (size as f64 + transformed_point.x) as u32;
        y = (size as f64 + transformed_point.y) as u32;

        println!("x = {}, y = {}", x, y);

        canvas.write(x, y, &Color::new(1., 1., 1.));
    }

    canvas.save();

    println!("Program took: {:?}", started_at.elapsed());
}
