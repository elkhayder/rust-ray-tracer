mod helpers;
mod models;

use std::{f64::consts::PI, time::Instant};

use crate::models::{
    axis::Axis,
    canvas::Canvas,
    colors::Color,
    intersection::{Intersection, Intersections},
    matrices::Matrices,
    rays::Ray,
    spheres::Sphere,
    tuples::Tuple,
};

fn main() {
    let started_at = Instant::now();

    let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

    let sphere = Sphere::new();

    let intersections = Intersections::new(vec![
        Intersection::new(&sphere, -0.1),
        Intersection::new(&sphere, -1.1),
        Intersection::new(&sphere, -0.1),
        Intersection::new(&sphere, -3.1),
    ]);

    println!("{:?}", intersections.hit());

    println!("Program took: {:?}", started_at.elapsed());
}
