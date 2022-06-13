use std::ops::{Deref, DerefMut};

use super::spheres::Sphere;

type Object = Sphere;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub object: Object,
    pub t: f64,
}

impl Intersection {
    pub fn new(object: &Object, t: f64) -> Self {
        Self { object: *object, t }
    }
}

/*  */

pub struct Intersections {
    items: Vec<Intersection>,
}

impl Deref for Intersections {
    type Target = Vec<Intersection>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Intersections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl Intersections {
    pub fn new(items: Vec<Intersection>) -> Self {
        Self { items }
    }

    pub fn hit(&self) -> Option<Intersection> {
        if self.is_empty() {
            return None;
        }

        let mut intersection: Option<Intersection> = None;

        self.iter().for_each(|current| {
            if current.t < 0.0 {
                return;
            }

            if intersection.is_none() || current.t < intersection.unwrap().t {
                intersection = Some(*current);
            }
        });

        intersection
    }
}
