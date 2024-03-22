pub mod functions;

#[derive(Copy, Clone)]
pub struct CartesianVector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
pub struct SphericalVector {
    r: f64,
    right_ascension: f64,
    declination: f64,
}

impl CartesianVector {
    pub fn new(x: f64, y: f64, z: f64) -> CartesianVector {
        CartesianVector { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn plus(&self, other: CartesianVector) -> CartesianVector {
        CartesianVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn minus(&self, other: CartesianVector) -> CartesianVector {
        CartesianVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn dot(&self, other: CartesianVector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn scale(&self, scalar: f64) -> CartesianVector {
        CartesianVector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn cross(&self, other: CartesianVector) -> CartesianVector {
        CartesianVector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> CartesianVector {
        let mag = self.magnitude();
        self.scale(1.0 / mag)
    }

    pub fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.x, self.y, self.z)
    }

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
}

impl SphericalVector {
    pub fn new(r: f64, right_ascension: f64, declination: f64) -> SphericalVector {
        SphericalVector {
            r,
            right_ascension,
            declination,
        }
    }

    pub fn to_cartesian(&self) -> CartesianVector {
        let x = self.r * self.right_ascension.cos() * self.declination.cos();
        let y = self.r * self.right_ascension.sin() * self.declination.cos();
        let z = self.r * self.declination.sin();
        CartesianVector { x, y, z }
    }

    pub fn to_string(&self) -> String {
        format!(
            "({}, {}, {})",
            self.r, self.right_ascension, self.declination
        )
    }
}

#[cfg(test)]
mod cartesian_vector_tests {
    use super::*;

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
}
