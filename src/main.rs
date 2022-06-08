mod helpers;
mod models;

use std::time::Instant;

use models::{canvas::Canvas, tuples::Tuple};

use crate::models::colors::Color;

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = &proj.position + &proj.velocity;
    proj.velocity = &(&proj.velocity + &env.gravity) + &env.wind;
}

fn main() {
    let started_at = Instant::now();

    let width = 1000;
    let height = 1000;

    let mut canvas = Canvas::new(width, height, None);

    let mut proj = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: &Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    while proj.position.y >= 0.0 {
        canvas.write(
            proj.position.x as u32,
            height - proj.position.y as u32,
            &Color::new(1.0, 1.0, 1.0),
        );
        tick(&env, &mut proj);
    }

    canvas.save();

    println!("Program took: {:?}", started_at.elapsed());
}
