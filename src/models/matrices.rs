use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut, Mul, Range},
};

use float_cmp::approx_eq;

use crate::helpers::constants::FLOATS_COMPARISON_ULPS;

use super::{axis::Axis, tuples::Tuple};

// Generators

pub struct Matrices {}

impl Matrices {
    pub fn square(size: u32, data: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: size,
            columns: size,
            data,
        }
    }

    pub fn identity(size: u32) -> Matrix {
        Matrix {
            rows: size,
            columns: size,
            data: (0..size)
                .map(|x| (0..size).map(|y| if x == y { 1.0 } else { 0.0 }).collect())
                .collect(),
        }
    }

    pub fn zeros(size: u32) -> Matrix {
        Matrix {
            rows: size,
            columns: size,
            data: (0..size)
                .map(|_| (0..size).map(|_| 0.0).collect())
                .collect(),
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrices::identity(4);

        matrix[0..3] = x;
        matrix[1..3] = y;
        matrix[2..3] = z;

        matrix
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrices::identity(4);

        matrix[0..0] = x;
        matrix[1..1] = y;
        matrix[2..2] = z;

        matrix
    }

    pub fn rotation(axis: Axis, r: f64) -> Matrix {
        let mut m = Matrices::identity(4);
        let cos = r.cos();
        let sin = r.sin();

        match axis {
            Axis::X => {
                m[1..1] = cos;
                m[2..2] = cos;
                m[2..1] = sin;
                m[1..2] = -sin;
            }
            Axis::Y => {
                m[0..0] = cos;
                m[2..2] = cos;
                m[0..2] = sin;
                m[2..0] = -sin;
            }
            Axis::Z => {
                m[0..0] = cos;
                m[1..1] = cos;
                m[0..1] = -sin;
                m[1..0] = sin;
            }
        }

        m
    }

    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
        let mut m = Matrices::identity(4);

        m[0..1] = x_y;
        m[0..2] = x_z;

        m[1..0] = y_x;
        m[1..2] = y_z;

        m[2..0] = z_x;
        m[2..1] = z_y;

        m
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: u32,
    pub columns: u32,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn transpose(&self) -> Matrix {
        Matrix {
            rows: self.columns,
            columns: self.rows,
            data: (0..self.rows)
                .map(|row| (0..self.columns).map(|column| self[column..row]).collect())
                .collect(),
        }
    }

    // Determinant for 2x2 matrix
    pub fn det(&self) -> f64 {
        let mut det = 0.0;

        if self.rows == 2 && self.columns == 2 {
            det = self[0..0] * self[1..1] - self[0..1] * self[1..0];
        } else {
            (0..self.columns).for_each(|c| det += self[0..c] * self.cofactor(0, c as usize));
        }

        det
    }

    pub fn sub_matrix(&self, row: usize, column: usize) -> Matrix {
        let mut matrix = Matrix {
            rows: self.rows - 1,
            columns: self.columns - 1,
            data: vec![vec![0.0; (self.columns - 1) as usize]; (self.rows - 1) as usize],
        };

        // TODO: Document this code since it looks horrible
        for (x, c_row) in self.data.iter().enumerate() {
            if x == row {
                continue;
            }

            for (y, c_column) in c_row.iter().enumerate() {
                if y == column {
                    continue;
                }

                matrix[(if x < row { x } else { x - 1 }) as u32..(if y < column {
                    y
                } else {
                    y - 1
                }) as u32] = *c_column;
            }
        }

        matrix
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.sub_matrix(row, column).det()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        (if row % 2 == column % 2 { 1.0 } else { -1.0 }) * self.minor(row, column)
    }

    pub fn is_invertible(&self) -> bool {
        !approx_eq!(f64, self.det(), 0.0, ulps = FLOATS_COMPARISON_ULPS)
    }

    pub fn inverse(&self) -> Matrix {
        let mut matrix = Matrix {
            rows: self.rows,
            columns: self.columns,
            data: vec![vec![0.0; self.columns as usize]; self.rows as usize],
        };

        let det = self.det();

        (0..self.rows).for_each(|row| {
            (0..self.columns)
                .for_each(|col| matrix[col..row] = self.cofactor(row as usize, col as usize) / det)
        });

        matrix
    }
}

// Debug

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix {0}x{1} \n{2}",
            self.rows,
            self.columns,
            self.data
                .iter()
                .map(|row| "|".to_string()
                    + row
                        .iter()
                        .map(|x| format!("{:.5}", x))
                        .collect::<Vec<String>>()
                        .join("|")
                        .as_str()
                    + "|")
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

/* Selectors */
impl Index<Range<u32>> for Matrix {
    type Output = f64;

    fn index(&self, index: Range<u32>) -> &Self::Output {
        &(self.data[index.start as usize])[index.end as usize]
    }
}

impl IndexMut<Range<u32>> for Matrix {
    fn index_mut(&mut self, index: Range<u32>) -> &mut Self::Output {
        &mut (self.data[index.start as usize])[index.end as usize]
    }
}

// Equality
impl<'a, 'b> PartialEq<&'b Matrix> for &'a Matrix {
    fn ne(&self, other: &&'b Matrix) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &&'b Matrix) -> bool {
        let mut eq = self.rows == other.rows && self.columns == other.columns;

        if !eq {
            return eq;
        }

        for y in 0..self.columns {
            for x in 0..self.rows {
                eq &= approx_eq!(f64, self[x..y], other[x..y], ulps = FLOATS_COMPARISON_ULPS);

                if !eq {
                    return eq;
                }
            }
        }

        eq
    }
}

// Multiplication
impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, other: &'b Matrix) -> Self::Output {
        assert!(
            self.columns == other.rows,
            "Tried {}x{} * {}x{} matrices multiplication",
            self.rows,
            self.columns,
            other.rows,
            other.columns,
        );

        let mut result = Matrix {
            rows: self.rows,
            columns: other.columns,
            data: vec![vec![0.0; other.columns as usize]; self.rows as usize],
        };

        let mut sum: f64;

        for row in 0..result.rows {
            for column in 0..result.columns {
                sum = 0.0;
                for i in 0..self.columns {
                    sum += self[row..i] * other[i..column];
                }
                result[row..column] = sum;
            }
        }

        result
    }
}

// Tuple casting
impl<'a> From<&'a Tuple> for Matrix {
    fn from(t: &'a Tuple) -> Self {
        Matrix {
            rows: 4,
            columns: 1,
            data: vec![vec![t.x], vec![t.y], vec![t.z], vec![t.w]],
        }
    }
}

// Tuple multiplication
impl<'a, 'b> Mul<&'b Tuple> for &'a Matrix {
    type Output = Tuple;

    fn mul(self, tuple: &'b Tuple) -> Self::Output {
        Tuple::from(&(self * &Matrix::from(tuple)))
    }
}
