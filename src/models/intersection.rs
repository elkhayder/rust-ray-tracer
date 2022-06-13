use super::spheres::Sphere;

type Object = Sphere;

pub struct Intersection {
    pub object: Object,
    pub t: f64,
}

impl Intersection {
    pub fn new(object: &Object, t: f64) -> Self {
        Self { object: *object, t }
    }
}
