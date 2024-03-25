//! A module for vector math
//! # Examples
//! ```rust
//! use linear_algebra::vectors::CartesianVector;
//! let v1 = CartesianVector::new(1.0, 2.0, 3.0);
//! let v2 = CartesianVector::new(4.0, 5.0, 6.0);
//! let v3 = v1.cross(v2);
//! println!("{:?}", v3);
//! ```
//! # Functions
//! * [add](CartesianVector::plus)
//! * subtract
//! * dot
//! * scale
//! * cross
//! * magnitude
//! * normalize
//! * to_spherical
//! * x_axis
//! * y_axis
//! * z_axis
//! * rotate_about_x
//! * rotate_about_y
//! * rotate_about_z
//! * rotate_about_axis
//!

/// A 3D vector with x, y, and z components
#[derive(Copy, Clone, Debug)]
pub struct CartesianVector {
    x: f64,
    y: f64,
    z: f64,
}

/// A 3D vector with r, right ascension, and declination components
#[derive(Copy, Clone, Debug)]
pub struct SphericalVector {
    r: f64,
    right_ascension: f64,
    declination: f64,
}

impl CartesianVector {
    /// Create a new CartesianVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// println!("{:?}", v1); // [1.0, 2.0, 3.0]
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> CartesianVector {
        CartesianVector { x, y, z }
    }

    /// Create a CartesianVector with x = 1.0, y = 0.0, z = 0.0
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::x_axis();
    /// println!("{:?}", v1); // [1.0, 0.0, 0.0]
    /// ```
    pub fn x_axis() -> CartesianVector {
        CartesianVector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Create a CartesianVector with x = 0.0, y = 1.0, z = 0.0
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::y_axis();
    /// println!("{:?}", v1); // [0.0, 1.0, 0.0]
    /// ```
    pub fn y_axis() -> CartesianVector {
        CartesianVector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    /// Create a CartesianVector with x = 0.0, y = 0.0, z = 1.0
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::z_axis();
    /// println!("{:?}", v1); // [0.0, 0.0, 1.0]
    /// ```
    pub fn z_axis() -> CartesianVector {
        CartesianVector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    /// Get the x component of the CartesianVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let x = v1.x(); // 1.0
    /// ```
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the y component of the CartesianVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let y = v1.y(); // 2.0
    /// ```
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get the z component of the CartesianVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let z = v1.z(); // 3.0
    /// ```
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Add two CartesianVectors
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = CartesianVector::new(4.0, 5.0, 6.0);
    /// let v3 = v1.plus(v2);
    /// println!("{:?}", v3); // [5.0, 7.0, 9.0]
    /// ```
    pub fn plus(&self, other: CartesianVector) -> CartesianVector {
        CartesianVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtract two CartesianVectors
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = CartesianVector::new(4.0, 5.0, 6.0);
    /// let v3 = v1.minus(v2);
    /// println!("{:?}", v3); // [-3.0, -3.0, -3.0]
    /// ```
    pub fn minus(&self, other: CartesianVector) -> CartesianVector {
        CartesianVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// Calculate the dot product of two CartesianVectors
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = CartesianVector::new(4.0, 5.0, 6.0);
    /// let v3 = v1.dot(v2);
    /// println!("{:?}", v3); // 32.0
    /// ```
    pub fn dot(&self, other: CartesianVector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Scale a CartesianVector by a scalar
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.scale(2.0);
    /// println!("{:?}", v2); // [2.0, 4.0, 6.0]
    /// ```
    pub fn scale(&self, scalar: f64) -> CartesianVector {
        CartesianVector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Calculate the cross product of two CartesianVectors
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = CartesianVector::new(4.0, 5.0, 6.0);
    /// let v3 = v1.cross(v2);
    /// println!("{:?}", v3); // [-3.0, 6.0, -3.0]
    /// ```
    pub fn cross(&self, other: CartesianVector) -> CartesianVector {
        CartesianVector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Calculate the magnitude of a CartesianVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.magnitude();
    /// println!("{:?}", v2); // 3.7416573867739413
    /// ```
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize a CartesianVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.normalize();
    /// println!("{:?}", v2); // [0.2672612419124244, 0.5345224838248488, 0.8017837257372732]
    /// ```
    pub fn normalize(&self) -> CartesianVector {
        let mag = self.magnitude();
        self.scale(1.0 / mag)
    }

    /// Convert a CartesianVector to a string
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.to_string();
    /// println!("{:?}", v2); // "[1, 2, 3]"
    /// ```
    pub fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.x, self.y, self.z)
    }

    /// Convert a CartesianVector to a SphericalVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::{CartesianVector, SphericalVector};
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.to_spherical();
    /// println!("{:?}", v2); // SphericalVector { r: 3.7416573867739413, right_ascension: 1.1071487177940904, declination: 0.6405223126794245 }
    /// ```
    pub fn to_spherical(&self) -> SphericalVector {
        let r = self.magnitude();
        let right_ascension = self.y.atan2(self.x);
        let declination = (self.z / r).acos();
        SphericalVector {
            r,
            right_ascension,
            declination,
        }
    }

    /// Rotate a CartesianVector about the x-axis
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.rotate_about_x(std::f64::consts::PI / 2.0);
    /// println!("{:?}", v2); // [1.0, -3.0, 2.0]
    /// ```
    pub fn rotate_about_x(&self, angle: f64) -> CartesianVector {
        let x = self.x;
        let y = self.y * angle.cos() - self.z * angle.sin();
        let z = self.y * angle.sin() + self.z * angle.cos();
        CartesianVector { x, y, z }
    }

    /// Rotate a CartesianVector about the y-axis
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.rotate_about_y(std::f64::consts::PI / 2.0);
    /// println!("{:?}", v2); // [3.0, 2.0, -1.0]
    /// ```
    pub fn rotate_about_y(&self, angle: f64) -> CartesianVector {
        let x = self.x * angle.cos() + self.z * angle.sin();
        let y = self.y;
        let z = -self.x * angle.sin() + self.z * angle.cos();
        CartesianVector { x, y, z }
    }

    /// Rotate a CartesianVector about the z-axis
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.rotate_about_z(std::f64::consts::PI / 2.0);
    /// println!("{:?}", v2); // [-2.0, 1.0, 3.0]
    /// ```
    pub fn rotate_about_z(&self, angle: f64) -> CartesianVector {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        let z = self.z;
        CartesianVector { x, y, z }
    }

    /// Rotate a CartesianVector about an arbitrary axis
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::CartesianVector;
    /// let v1 = CartesianVector::new(1.0, 2.0, 3.0);
    /// let v2 = CartesianVector::new(1.0, 1.0, 1.0).normalize();
    /// let v3 = v1.rotate_about_axis(v2, std::f64::consts::PI / 2.0);
    /// println!("{:?}", v3); // [2.577350269189626, 0.8452994616207488, 2.577350269189626]
    /// ```
    pub fn rotate_about_axis(&self, axis: CartesianVector, angle: f64) -> CartesianVector {
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        let x = self.x * (c + axis.x * axis.x * t)
            + self.y * (axis.x * axis.y * t - axis.z * s)
            + self.z * (axis.x * axis.z * t + axis.y * s);
        let y = self.x * (axis.y * axis.x * t + axis.z * s)
            + self.y * (c + axis.y * axis.y * t)
            + self.z * (axis.y * axis.z * t - axis.x * s);
        let z = self.x * (axis.z * axis.x * t - axis.y * s)
            + self.y * (axis.z * axis.y * t + axis.x * s)
            + self.z * (c + axis.z * axis.z * t);
        CartesianVector { x, y, z }
    }
}

impl SphericalVector {
    /// Create a new SphericalVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::SphericalVector;
    /// let v1 = SphericalVector::new(1.0, 2.0, 3.0);
    /// println!("{:?}", v1); // SphericalVector { r: 1.0, right_ascension: 2.0, declination: 3.0 }
    /// ```
    pub fn new(r: f64, right_ascension: f64, declination: f64) -> SphericalVector {
        SphericalVector {
            r,
            right_ascension,
            declination,
        }
    }

    /// Convert a SphericalVector to a CartesianVector
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::{CartesianVector, SphericalVector};
    /// let v1 = SphericalVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.to_cartesian();
    /// println!("{:?}", v2); // [0.4025674938050524, 0.6142127126896676, 0.6795203866473763]
    /// ```
    pub fn to_cartesian(&self) -> CartesianVector {
        let x = self.r * self.right_ascension.cos() * self.declination.cos();
        let y = self.r * self.right_ascension.sin() * self.declination.cos();
        let z = self.r * self.declination.sin();
        CartesianVector { x, y, z }
    }

    /// Convert a SphericalVector to a string
    /// # Example
    /// ```rust
    /// use linear_algebra::vectors::SphericalVector;
    /// let v1 = SphericalVector::new(1.0, 2.0, 3.0);
    /// let v2 = v1.to_string();
    /// println!("{:?}", v2); // "[1, 2, 3]"
    /// ```
    pub fn to_string(&self) -> String {
        format!(
            "[{}, {}, {}]",
            self.r, self.right_ascension, self.declination
        )
    }
}

#[cfg(test)]
mod cartesian_vector_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_cartesian_vector_plus() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = CartesianVector::new(4.0, 5.0, 6.0);
        let c = a.plus(b);
        assert_eq!(c.x, 5.0);
        assert_eq!(c.y, 7.0);
        assert_eq!(c.z, 9.0);
    }

    #[test]
    fn test_cartesian_vector_minus() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = CartesianVector::new(4.0, 5.0, 6.0);
        let c = a.minus(b);
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, -3.0);
        assert_eq!(c.z, -3.0);
    }

    #[test]
    fn test_cartesian_vector_dot() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = CartesianVector::new(4.0, 5.0, 6.0);
        let c = a.dot(b);
        assert_eq!(c, 32.0);
    }

    #[test]
    fn test_cartesian_vector_scale() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = a.scale(2.0);
        assert_eq!(b.x, 2.0);
        assert_eq!(b.y, 4.0);
        assert_eq!(b.z, 6.0);
    }

    #[test]
    fn test_cartesian_vector_cross() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = CartesianVector::new(4.0, 5.0, 6.0);
        let c = a.cross(b);
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, 6.0);
        assert_eq!(c.z, -3.0);
    }

    #[test]
    fn test_cartesian_vector_magnitude() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = a.magnitude();
        assert_eq!(b, 3.7416573867739413);
    }

    #[test]
    fn test_cartesian_vector_normalize() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = a.normalize();
        assert_eq!(b.x, 0.2672612419124244);
        assert_eq!(b.y, 0.5345224838248488);
        assert_eq!(b.z, 0.8017837257372732);
    }

    #[test]
    fn test_cartesian_vector_to_string() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = a.to_string();
        assert_eq!(b, "[1, 2, 3]");
    }

    #[test]
    fn test_cartesian_vector_to_spherical() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = a.to_spherical();
        assert_eq!(b.r, 3.7416573867739413);
        assert_eq!(b.right_ascension, 1.1071487177940904);
        assert_eq!(b.declination, 0.6405223126794245);
    }

    #[test]
    fn test_cartesian_vector_x_axis() {
        let a = CartesianVector::x_axis();
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 0.0);
        assert_eq!(a.z, 0.0);
    }

    #[test]
    fn test_cartesian_vector_y_axis() {
        let a = CartesianVector::y_axis();
        assert_eq!(a.x, 0.0);
        assert_eq!(a.y, 1.0);
        assert_eq!(a.z, 0.0);
    }

    #[test]
    fn test_cartesian_vector_z_axis() {
        let a = CartesianVector::z_axis();
        assert_eq!(a.x, 0.0);
        assert_eq!(a.y, 0.0);
        assert_eq!(a.z, 1.0);
    }

    #[test]
    fn test_cartesian_vector_rotate_about_x() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = a.rotate_about_x(std::f64::consts::PI / 2.0);
        assert_approx_eq!(b.x, 1.0, 1e-12);
        assert_approx_eq!(b.y, -3.0, 1e-12);
        assert_approx_eq!(b.z, 2.0, 1e-12);
    }

    #[test]
    fn test_cartesian_vector_rotate_about_y() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = a.rotate_about_y(std::f64::consts::PI / 2.0);
        assert_approx_eq!(b.x, 3.0, 1e-12);
        assert_approx_eq!(b.y, 2.0, 1e-12);
        assert_approx_eq!(b.z, -1.0, 1e-12);
    }

    #[test]
    fn test_cartesian_vector_rotate_about_z() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = a.rotate_about_z(std::f64::consts::PI / 2.0);
        assert_approx_eq!(b.x, -2.0, 1e-12);
        assert_approx_eq!(b.y, 1.0, 1e-12);
        assert_approx_eq!(b.z, 3.0, 1e-12);
    }

    #[test]
    fn test_cartesian_vector_rotate_about_axis() {
        let a = CartesianVector::new(1.0, 2.0, 3.0);
        let b = CartesianVector::new(1.0, 1.0, 1.0).normalize();
        let c = a.rotate_about_axis(b, std::f64::consts::PI / 2.0);
        assert_approx_eq!(c.x, 2.577350269189626, 1e-12);
        assert_approx_eq!(c.y, 0.8452994616207488, 1e-12);
        assert_approx_eq!(c.z, 2.577350269189626, 1e-12);
    }
}
