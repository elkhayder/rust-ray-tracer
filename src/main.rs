mod helpers;
mod models;

use std::{f64::consts::PI, time::Instant};

use crate::models::{
    axis::Axis, canvas::Canvas, colors::Color, matrices::Matrices, rays::Ray, spheres::Sphere,
    tuples::Tuple,
};

fn main() {
    let started_at = Instant::now();

    let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

    let sphere = Sphere::new();

    let intersections = sphere.intersect(&ray);

    println!("Found {} intersection(s)", intersections.len());

    intersections.iter().for_each(|intersection| {
        println!(
            "Intersection at t = {}, {}",
            intersection.t,
            ray.position(intersection.t)
        )
    });

    println!("Program took: {:?}", started_at.elapsed());
}
