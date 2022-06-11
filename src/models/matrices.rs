use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut, Mul, Range},
};

use float_cmp::approx_eq;

use crate::helpers::constants::FLOATS_COMPARISON_ULPS;

use super::tuples::Tuple;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: u32,
    pub columns: u32,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn square(size: u32, data: Vec<Vec<f64>>) -> Matrix {
        // assert!(
        //     size as usize == data.len(),
        //     "Matrix columns aren't equal to its size"
        // );z

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
