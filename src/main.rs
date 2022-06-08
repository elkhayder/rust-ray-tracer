mod models;

use crate::models::tuples::Tuple;

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    println!("Position: {}, Velocity: {}", proj.position, proj.velocity);
    proj.position = &proj.position + &proj.velocity;
    proj.velocity = &(&proj.velocity + &env.gravity) + &env.wind;
}

fn main() {
    let mut proj = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    while proj.position.y >= 0.0 {
        tick(&env, &mut proj);
    }
}
