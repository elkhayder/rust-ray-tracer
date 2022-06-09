use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

use float_cmp::approx_eq;

use super::matrices::Matrix;

pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag: f64 = self.magnitude();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

// Constructors
impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1f64 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0f64 }
    }
}

// Debug
impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tuple: {} {{x: {}, y: {}, z: {}, w: {}}}",
            if self.w == 1f64 {
                "Point"
            } else if self.w == 0f64 {
                "Vector"
            } else {
                "Unknown"
            },
            self.x,
            self.y,
            self.z,
            self.w
        )
    }
}

/* OPERATORS */

// Addition
impl<'a, 'b> Add<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn add(self, other: &'b Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

// Subtraction
impl<'a, 'b> Sub<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn sub(self, other: &'b Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

// Negate
impl<'a> Neg for &'a Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w, // TODO: Check if you'll have to invert W too
        }
    }
}

// Multiply by number
impl<'a> Mul<f64> for &'a Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other, // TODO: Check if you'll have to multiply w too
        }
    }
}

// Divide by number
impl<'a> Div<f64> for &'a Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other, // TODO: Check if you'll have to divide w too
        }
    }
}

// Cross product
impl<'a, 'b> Mul<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn mul(self, other: &'b Tuple) -> Self::Output {
        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: self.w,
        }
    }
}

/* Comparison */
impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        const ULPS: i64 = 2;
        approx_eq!(f64, self.x, other.x, ulps = ULPS)
            && approx_eq!(f64, self.y, other.y, ulps = ULPS)
            && approx_eq!(f64, self.z, other.z, ulps = ULPS)
            && approx_eq!(f64, self.w, other.w, ulps = ULPS)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Multiply by a matrix

// impl<'a, 'b> Mul<&'b Matrix> for &'a Tuple {
//     type Output = Tuple;

//     fn mul(self, other: &'b Tuple) -> Self::Output {
//         Tuple {
//             x: self.y * other.z - self.z * other.y,
//             y: self.z * other.x - self.x * other.z,
//             z: self.x * other.y - self.y * other.x,
//             w: self.w,
//         }
//     }
// }
