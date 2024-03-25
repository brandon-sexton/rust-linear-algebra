#![warn(missing_docs)]
//! # Overview
//! A basic linear algebra library for vector and matrix math
//!
//! # Examples
//! ```rust
//! use linear_algebra::vectors::CartesianVector;
//!
//! let v1 = CartesianVector::new(1.0, 2.0, 3.0);
//! let v2 = CartesianVector::new(4.0, 5.0, 6.0);
//! let v3 = v1.cross(v2);
//! println!("{:?}", v3);
//! ```
pub mod matrices;
pub mod vectors;
