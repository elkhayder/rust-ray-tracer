use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn format(&self) -> String {
        format!(
            "{} {} {}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }
}

/* OPERATIONS */

// Addition
impl<'a, 'b> Add<&'b Color> for &'a Color {
    type Output = Color;

    fn add(self, other: &'b Color) -> Self::Output {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

// Subtraction
impl<'a, 'b> Sub<&'b Color> for &'a Color {
    type Output = Color;

    fn sub(self, other: &'b Color) -> Self::Output {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

// Multiplying by another color
impl<'a, 'b> Mul<&'b Color> for &'a Color {
    type Output = Color;

    fn mul(self, other: &'b Color) -> Self::Output {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

// Multiplying by a scalar
impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Self::Output {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}
