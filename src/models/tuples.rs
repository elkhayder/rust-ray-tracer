use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

use float_cmp::approx_eq;

use crate::helpers::constants::FLOATS_COMPARISON_ULPS;

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

    pub fn apply(&self, matrices: &[&Matrix]) -> Tuple {
        Tuple::from(
            &matrices
                .into_iter()
                .fold(Matrix::from(self), |prev, item| *item * &prev),
        )
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
impl Add<&Tuple> for &Tuple {
    type Output = Tuple;

    fn add(self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

// Subtraction
impl Sub<&Tuple> for &Tuple {
    type Output = Tuple;

    fn sub(self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

// Negate
impl Neg for &Tuple {
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
impl Mul<f64> for &Tuple {
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
impl Div<f64> for &Tuple {
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
impl Mul<Tuple> for &Tuple {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
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
        approx_eq!(f64, self.x, other.x, ulps = FLOATS_COMPARISON_ULPS)
            && approx_eq!(f64, self.y, other.y, ulps = FLOATS_COMPARISON_ULPS)
            && approx_eq!(f64, self.z, other.z, ulps = FLOATS_COMPARISON_ULPS)
            && approx_eq!(f64, self.w, other.w, ulps = FLOATS_COMPARISON_ULPS)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Matrix casting

impl From<&Matrix> for Tuple {
    fn from(matrix: &Matrix) -> Self {
        assert!(matrix.rows == 4 && matrix.columns == 1);

        Tuple {
            x: matrix[0..0],
            y: matrix[1..0],
            z: matrix[2..0],
            w: matrix[3..0],
        }
    }
}
