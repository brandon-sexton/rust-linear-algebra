//! # 3D Vector Functions

/// Add two 3D vectors
pub fn add_3d(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// Subtract two 3D vectors
pub fn subtract_3d(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Dot product of two 3D vectors
pub fn dot_3d(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Cross product of two 3D vectors
pub fn cross_3d(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Multiply a 3D vector by a scalar
pub fn scale_3d(a: [f64; 3], scalar: f64) -> [f64; 3] {
    [a[0] * scalar, a[1] * scalar, a[2] * scalar]
}

/// Magnitude of a 3D vector
pub fn magnitude_3d(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

/// Normalize a 3D vector
pub fn normalize_3d(a: [f64; 3]) -> [f64; 3] {
    let mag = magnitude_3d(a);
    scale_3d(a, 1.0 / mag)
}
