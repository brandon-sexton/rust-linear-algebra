/// Represents a 3D vector with x, y, and z components.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    /// Creates a new Vector3D with the given x, y, and z components.
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    /// Calculates the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculates the dot product of this vector and another vector.
    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the cross product of this vector and another vector.
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Adds this vector to another vector.
    /// Returns a new vector with the result.
    pub fn plus(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtracts another vector from this vector.
    /// Returns a new vector with the result.
    pub fn minus(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// Gets the element at the given index.
    pub fn element(&self, index: usize) -> f64 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_new() {
        let vector = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
    }

    #[test]
    fn test_magnitude() {
        let vector = Vector3D::new(3.0, 4.0, 5.0);
        assert_approx_eq!(vector.magnitude(), 7.071067811865476, 1e-12);
    }

    #[test]
    fn test_dot() {
        let vector1 = Vector3D::new(1.0, 2.0, 3.0);
        let vector2 = Vector3D::new(4.0, 5.0, 6.0);
        assert_eq!(vector1.dot(&vector2), 32.0);
    }

    #[test]
    fn test_cross() {
        let vector1 = Vector3D::new(1.0, 2.0, 3.0);
        let vector2 = Vector3D::new(4.0, 5.0, 6.0);
        let cross_product = vector1.cross(&vector2);
        assert_eq!(cross_product.x, -3.0);
        assert_eq!(cross_product.y, 6.0);
        assert_eq!(cross_product.z, -3.0);
    }

    #[test]
    fn test_plus() {
        let vector1 = Vector3D::new(1.0, 2.0, 3.0);
        let vector2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = vector1.plus(&vector2);
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_minus() {
        let vector1 = Vector3D::new(1.0, 2.0, 3.0);
        let vector2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = vector1.minus(&vector2);
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, -3.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_element() {
        let vector = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(vector.element(0), 1.0);
        assert_eq!(vector.element(1), 2.0);
        assert_eq!(vector.element(2), 3.0);
    }
}
