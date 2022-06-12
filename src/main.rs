mod helpers;
mod models;

use std::{f64::consts::PI, time::Instant};

use crate::models::{
    axis::Axis, canvas::Canvas, colors::Color, matrices::Matrices, rays::Ray, tuples::Tuple,
};

fn main() {
    let started_at = Instant::now();

    let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.15, 2.0, 3.0));

    let t = 3.0;

    println!("Position where t = {}, {}", t, ray.position(t));

    println!("Program took: {:?}", started_at.elapsed());
}
