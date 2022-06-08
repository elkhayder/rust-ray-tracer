mod models;

use crate::models::tuples::Tuple;

fn main() {
    let p = Tuple::point(1f64, 1f64, 1f64);
    let a = Tuple::vector(1f64, 2f64, 3f64);
    let b = Tuple::vector(2f64, 3f64, 4f64);

    // println!("x + y = {}", &p + &v);
    println!("p - a = {}", &p - &a);
    println!("a magnitude = {}", p.magnitude());
    println!("a normalized = {}", p.normalize());
    println!("p / 2 = {}", &p / 2f64);
    println!("p * 0.5 = {}", &p * 0.5f64);
    println!("p / 2 == p * 0.5 is {}", &p / 2f64 == &p * 0.5f64);
    println!("a . b = {}", &a.dot(&b));
    println!("a * b = {}", &a * &b);
    println!("b * a = {}", &b * &a);
}
