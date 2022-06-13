use super::{
    intersection::{Intersection, Intersections},
    rays::Ray,
    tuples::Tuple,
};

#[derive(Debug, Clone, Copy)]
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

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut r = Intersections::new(vec![]);

        let sphere_to_ray = &ray.origin - &self.origin;

        let a = ray.direction.dot(&ray.direction);

        let b = 2.0 * &ray.direction.dot(&sphere_to_ray);

        let c = &sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return r;
        }

        r.push(Intersection::new(
            self,
            (-b - discriminant.sqrt()) / (2.0 * a),
        ));

        r.push(Intersection::new(
            self,
            (-b + discriminant.sqrt()) / (2.0 * a),
        ));

        r
    }
}
