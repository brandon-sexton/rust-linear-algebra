//! # Matrix 2x2 functions

/// Get the determinant of a 2x2 matrix
pub fn determinant_2x2(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}
