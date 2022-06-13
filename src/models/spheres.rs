use super::{rays::Ray, tuples::Tuple};

pub struct Sphere {
    pub origin: Tuple,
    // pub radius: f64,
}

impl Sphere {
    pub fn new() -> Sphere {
        Self {
            origin: Tuple::point(0.0, 0.0, 0.0),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let mut r = vec![];

        let sphere_to_ray = &ray.origin - &self.origin;

        let a = ray.direction.dot(&ray.direction);

        let b = 2.0 * &ray.direction.dot(&sphere_to_ray);

        let c = &sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return r;
        }

        r.push((-b - discriminant.sqrt()) / (2.0 * a));
        r.push((-b + discriminant.sqrt()) / (2.0 * a));

        r
    }
}
