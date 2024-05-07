//! 2D vector functions

/// Add two 2D vectors
pub fn add_2d(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

/// Subtract two 2D vectors
pub fn subtract_2d(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

/// Dot product of two 2D vectors
pub fn dot_2d(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

/// Scale a 2D vector by a scalar
pub fn scale_2d(a: [f64; 2], scalar: f64) -> [f64; 2] {
    [a[0] * scalar, a[1] * scalar]
}

/// Magnitude of a 2D vector
pub fn magnitude_2d(a: [f64; 2]) -> f64 {
    (a[0] * a[0] + a[1] * a[1]).sqrt()
}

/// Normalize a 2D vector
pub fn normalize_2d(a: &[f64; 2]) -> [f64; 2] {
    let mag = magnitude_2d(*a);
    scale_2d(*a, 1.0 / mag)
}
