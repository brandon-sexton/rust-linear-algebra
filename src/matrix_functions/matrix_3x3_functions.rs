//! # Matrix 3x3 functions

use crate::matrix_functions::matrix_2x2_functions::determinant_2x2;
use crate::vector_functions::vector_3d_functions::*;

/// Add two 3x3 matrices
pub fn add_3x3(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [add_3d(a[0], b[0]), add_3d(a[1], b[1]), add_3d(a[2], b[2])]
}

/// Subtract two 3x3 matrices
pub fn subtract_3x3(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [
        subtract_3d(a[0], b[0]),
        subtract_3d(a[1], b[1]),
        subtract_3d(a[2], b[2]),
    ]
}

/// Multiply two 3x3 matrices
pub fn multiply_3x3(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [
        multiply_1x3(a[0], b),
        multiply_1x3(a[1], b),
        multiply_1x3(a[2], b),
    ]
}

/// Multiply a 3D vector by a 3x3 matrix
pub fn multiply_1x3(a: [f64; 3], b: [[f64; 3]; 3]) -> [f64; 3] {
    [
        dot_3d(a, column_3d(b, 0)),
        dot_3d(a, column_3d(b, 1)),
        dot_3d(a, column_3d(b, 2)),
    ]
}

/// Get the column of a 3x3 matrix as a 3D vector
pub fn column_3d(a: [[f64; 3]; 3], i: usize) -> [f64; 3] {
    [a[0][i], a[1][i], a[2][i]]
}

/// Transpose a 3x3 matrix
pub fn transpose_3x3(a: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [column_3d(a, 0), column_3d(a, 1), column_3d(a, 2)]
}

/// Scale a 3x3 matrix by a scalar
pub fn scale_3x3(a: [[f64; 3]; 3], b: f64) -> [[f64; 3]; 3] {
    [scale_3d(a[0], b), scale_3d(a[1], b), scale_3d(a[2], b)]
}

/// Get the determinant of a 3x3 matrix
pub fn determinant_3x3(a: [[f64; 3]; 3]) -> f64 {
    a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])
}

/// Get the minor matrix of a 3x3 matrix
pub fn minor_matrix_3x3(a: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [
        [
            determinant_2x2([a[1][1], a[1][2]], [a[2][1], a[2][2]]),
            determinant_2x2([a[1][0], a[1][2]], [a[2][0], a[2][2]]),
            determinant_2x2([a[1][0], a[1][1]], [a[2][0], a[2][1]]),
        ],
        [
            determinant_2x2([a[0][1], a[0][2]], [a[2][1], a[2][2]]),
            determinant_2x2([a[0][0], a[0][2]], [a[2][0], a[2][2]]),
            determinant_2x2([a[0][0], a[0][1]], [a[2][0], a[2][1]]),
        ],
        [
            determinant_2x2([a[0][1], a[0][2]], [a[1][1], a[1][2]]),
            determinant_2x2([a[0][0], a[0][2]], [a[1][0], a[1][2]]),
            determinant_2x2([a[0][0], a[0][1]], [a[1][0], a[1][1]]),
        ],
    ]
}

/// Get the cofactor matrix of a 3x3 matrix
pub fn cofactor_matrix_3x3(a: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let minor = minor_matrix_3x3(a);
    [
        [minor[0][0], -minor[0][1], minor[0][2]],
        [-minor[1][0], minor[1][1], -minor[1][2]],
        [minor[2][0], -minor[2][1], minor[2][2]],
    ]
}

/// Get the adjugate matrix of a 3x3 matrix
pub fn adjugate_matrix_3x3(a: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    transpose_3x3(cofactor_matrix_3x3(a))
}

/// Get the inverse of a 3x3 matrix
pub fn inverse_3x3(a: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    scale_3x3(adjugate_matrix_3x3(a), 1.0 / determinant_3x3(a))
}
