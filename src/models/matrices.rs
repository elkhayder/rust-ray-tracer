use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut, Range},
};

use float_cmp::approx_eq;

#[derive(Debug, Clone)]
pub struct Matrix {
    size: u32,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(size: u32, data: Vec<Vec<f64>>) -> Matrix {
        // assert!(
        //     size as usize == data.len(),
        //     "Matrix columns aren't equal to its size"
        // );

        Matrix { size, data }
    }
}

// Debug

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix {0}x{0} \n{1}",
            self.size,
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
        let mut eq = self.size == other.size;

        if !eq {
            return eq;
        }

        for y in 0..self.size {
            for x in 0..self.size {
                eq &= approx_eq!(f64, self[x..y], other[x..y], ulps = 2);

                if !eq {
                    return eq;
                }
            }
        }

        eq
    }
}
