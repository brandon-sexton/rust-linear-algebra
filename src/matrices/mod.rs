//! Top-level module for general matrix operations

pub mod functions;
use super::vectors::CartesianVector;

/// A struct representing a 3x3 matrix in Cartesian space
pub struct CartesianMatrix {
    row_1: CartesianVector,
    row_2: CartesianVector,
    row_3: CartesianVector,
}

impl CartesianMatrix {
    /// Creates a new CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    /// }
    /// ```
    pub fn new(
        row_1: CartesianVector,
        row_2: CartesianVector,
        row_3: CartesianVector,
    ) -> CartesianMatrix {
        CartesianMatrix {
            row_1,
            row_2,
            row_3,
        }
    }

    /// Adds two CartesianMatrices
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1_a = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2_a = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3_a = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix_a = CartesianMatrix::new(row_1_a, row_2_a, row_3_a);
    ///     let row_1_b = CartesianVector::new(9.0, 8.0, 7.0);
    ///     let row_2_b = CartesianVector::new(6.0, 5.0, 4.0);
    ///     let row_3_b = CartesianVector::new(3.0, 2.0, 1.0);
    ///     let matrix_b = CartesianMatrix::new(row_1_b, row_2_b, row_3_b);
    ///     let result = matrix_a.plus(matrix_b);
    /// }
    /// ```
    pub fn plus(&self, other: CartesianMatrix) -> CartesianMatrix {
        CartesianMatrix {
            row_1: self.row_1.plus(other.row_1),
            row_2: self.row_2.plus(other.row_2),
            row_3: self.row_3.plus(other.row_3),
        }
    }

    /// Subtracts two CartesianMatrices
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1_a = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2_a = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3_a = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix_a = CartesianMatrix::new(row_1_a, row_2_a, row_3_a);
    ///     let row_1_b = CartesianVector::new(9.0, 8.0, 7.0);
    ///     let row_2_b = CartesianVector::new(6.0, 5.0, 4.0);
    ///     let row_3_b = CartesianVector::new(3.0, 2.0, 1.0);
    ///     let matrix_b = CartesianMatrix::new(row_1_b, row_2_b, row_3_b);
    ///     let result = matrix_a.minus(matrix_b);
    /// }
    /// ```
    pub fn minus(&self, other: CartesianMatrix) -> CartesianMatrix {
        CartesianMatrix {
            row_1: self.row_1.minus(other.row_1),
            row_2: self.row_2.minus(other.row_2),
            row_3: self.row_3.minus(other.row_3),
        }
    }

    /// Scales a CartesianMatrix by an input multiple
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.scale(2.0);
    /// }
    /// ```
    pub fn scale(&self, scalar: f64) -> CartesianMatrix {
        CartesianMatrix {
            row_1: self.row_1.scale(scalar),
            row_2: self.row_2.scale(scalar),
            row_3: self.row_3.scale(scalar),
        }
    }

    /// Returns a column of a CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.column(0);
    /// }
    /// ```
    pub fn column(&self, index: usize) -> CartesianVector {
        match index {
            0 => CartesianVector::new(self.row_1.x(), self.row_2.x(), self.row_3.x()),
            1 => CartesianVector::new(self.row_1.y(), self.row_2.y(), self.row_3.y()),
            2 => CartesianVector::new(self.row_1.z(), self.row_2.z(), self.row_3.z()),
            _ => panic!("Invalid column index"),
        }
    }

    /// Returns the first row of the calling CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.row_1();
    /// }
    /// ```
    pub fn row_1(&self) -> CartesianVector {
        self.row_1
    }

    /// Returns the second row of the calling CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.row_2();
    /// }
    /// ```
    pub fn row_2(&self) -> CartesianVector {
        self.row_2
    }

    /// Returns the third row of the calling CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.row_3();
    /// }
    /// ```
    pub fn row_3(&self) -> CartesianVector {
        self.row_3
    }

    /// Returns the first column of the calling CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.column_1();
    /// }
    /// ```
    pub fn column_1(&self) -> CartesianVector {
        self.column(0)
    }

    /// Returns the second column of the calling CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.column_2();
    /// }
    /// ```
    pub fn column_2(&self) -> CartesianVector {
        self.column(1)
    }

    /// Returns the third column of the calling CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.column_3();
    /// }
    /// ```
    pub fn column_3(&self) -> CartesianVector {
        self.column(2)
    }

    /// Returns the determinant of the calling CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.determinant();
    /// }
    /// ```
    pub fn determinant(&self) -> f64 {
        self.row_1.x() * (self.row_2.y() * self.row_3.z() - self.row_2.z() * self.row_3.y())
            - self.row_1.y() * (self.row_2.x() * self.row_3.z() - self.row_2.z() * self.row_3.x())
            + self.row_1.z() * (self.row_2.x() * self.row_3.y() - self.row_2.y() * self.row_3.x())
    }

    /// Returns the transpose of the calling CartesianMatrix
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let result = matrix.transpose();
    /// }
    /// ```
    pub fn transpose(&self) -> CartesianMatrix {
        CartesianMatrix {
            row_1: CartesianVector::new(self.row_1.x(), self.row_2.x(), self.row_3.x()),
            row_2: CartesianVector::new(self.row_1.y(), self.row_2.y(), self.row_3.y()),
            row_3: CartesianVector::new(self.row_1.z(), self.row_2.z(), self.row_3.z()),
        }
    }

    /// Multiplies two CartesianMatrices
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1_a = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2_a = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3_a = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix_a = CartesianMatrix::new(row_1_a, row_2_a, row_3_a);
    ///     let row_1_b = CartesianVector::new(9.0, 8.0, 7.0);
    ///     let row_2_b = CartesianVector::new(6.0, 5.0, 4.0);
    ///     let row_3_b = CartesianVector::new(3.0, 2.0, 1.0);
    ///     let matrix_b = CartesianMatrix::new(row_1_b, row_2_b, row_3_b);
    ///     let result = matrix_a.multiply_matrix(matrix_b);
    /// }
    /// ```
    pub fn multiply_matrix(&self, other: CartesianMatrix) -> CartesianMatrix {
        CartesianMatrix {
            row_1: CartesianVector::new(
                self.row_1.dot(other.column_1()),
                self.row_1.dot(other.column_2()),
                self.row_1.dot(other.column_3()),
            ),
            row_2: CartesianVector::new(
                self.row_2.dot(other.column_1()),
                self.row_2.dot(other.column_2()),
                self.row_2.dot(other.column_3()),
            ),
            row_3: CartesianVector::new(
                self.row_3.dot(other.column_1()),
                self.row_3.dot(other.column_2()),
                self.row_3.dot(other.column_3()),
            ),
        }
    }

    /// Multiplies a CartesianMatrix by a CartesianVector
    /// # Example
    /// ## Code
    /// ```rust
    /// use linear_algebra::matrices::CartesianMatrix;
    /// use linear_algebra::vectors::CartesianVector;
    ///
    /// fn main() {
    ///     let row_1 = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let row_2 = CartesianVector::new(4.0, 5.0, 6.0);
    ///     let row_3 = CartesianVector::new(7.0, 8.0, 9.0);
    ///     let matrix = CartesianMatrix::new(row_1, row_2, row_3);
    ///     let vector = CartesianVector::new(1.0, 2.0, 3.0);
    ///     let result = matrix.multiply_vector(vector);
    /// }
    /// ```
    pub fn multiply_vector(&self, other: CartesianVector) -> CartesianVector {
        CartesianVector::new(
            self.row_1.dot(other),
            self.row_2.dot(other),
            self.row_3.dot(other),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_matrix_plus() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = CartesianMatrix::new(
            CartesianVector::new(9.0, 8.0, 7.0),
            CartesianVector::new(6.0, 5.0, 4.0),
            CartesianVector::new(3.0, 2.0, 1.0),
        );
        let c = a.plus(b);
        assert_eq!(c.row_1().x(), 10.0);
        assert_eq!(c.row_1().y(), 10.0);
        assert_eq!(c.row_1().z(), 10.0);
        assert_eq!(c.row_2().x(), 10.0);
        assert_eq!(c.row_2().y(), 10.0);
        assert_eq!(c.row_2().z(), 10.0);
        assert_eq!(c.row_3().x(), 10.0);
        assert_eq!(c.row_3().y(), 10.0);
        assert_eq!(c.row_3().z(), 10.0);
    }

    #[test]
    fn test_cartesian_matrix_minus() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = CartesianMatrix::new(
            CartesianVector::new(9.0, 8.0, 7.0),
            CartesianVector::new(6.0, 5.0, 4.0),
            CartesianVector::new(3.0, 2.0, 1.0),
        );
        let c = a.minus(b);
        assert_eq!(c.row_1().x(), -8.0);
        assert_eq!(c.row_1().y(), -6.0);
        assert_eq!(c.row_1().z(), -4.0);
        assert_eq!(c.row_2().x(), -2.0);
        assert_eq!(c.row_2().y(), 0.0);
        assert_eq!(c.row_2().z(), 2.0);
        assert_eq!(c.row_3().x(), 4.0);
        assert_eq!(c.row_3().y(), 6.0);
        assert_eq!(c.row_3().z(), 8.0);
    }

    #[test]
    fn test_cartesian_matrix_scale() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.scale(2.0);
        assert_eq!(b.row_1().x(), 2.0);
        assert_eq!(b.row_1().y(), 4.0);
        assert_eq!(b.row_1().z(), 6.0);
        assert_eq!(b.row_2().x(), 8.0);
        assert_eq!(b.row_2().y(), 10.0);
        assert_eq!(b.row_2().z(), 12.0);
        assert_eq!(b.row_3().x(), 14.0);
        assert_eq!(b.row_3().y(), 16.0);
        assert_eq!(b.row_3().z(), 18.0);
    }

    #[test]
    fn test_cartesian_matrix_column() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.column(0);
        assert_eq!(b.x(), 1.0);
        assert_eq!(b.y(), 4.0);
        assert_eq!(b.z(), 7.0);
        let c = a.column(1);
        assert_eq!(c.x(), 2.0);
        assert_eq!(c.y(), 5.0);
        assert_eq!(c.z(), 8.0);
        let d = a.column(2);
        assert_eq!(d.x(), 3.0);
        assert_eq!(d.y(), 6.0);
        assert_eq!(d.z(), 9.0);
    }

    #[test]
    fn test_cartesian_matrix_determinant() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.determinant();
        assert_eq!(b, 0.0);
    }

    #[test]
    fn test_cartesian_matrix_transpose() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.transpose();
        assert_eq!(b.row_1().x(), 1.0);
        assert_eq!(b.row_1().y(), 4.0);
        assert_eq!(b.row_1().z(), 7.0);
        assert_eq!(b.row_2().x(), 2.0);
        assert_eq!(b.row_2().y(), 5.0);
        assert_eq!(b.row_2().z(), 8.0);
        assert_eq!(b.row_3().x(), 3.0);
        assert_eq!(b.row_3().y(), 6.0);
        assert_eq!(b.row_3().z(), 9.0);
    }

    #[test]
    fn test_cartesian_matrix_multiply_matrix() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = CartesianMatrix::new(
            CartesianVector::new(9.0, 8.0, 7.0),
            CartesianVector::new(6.0, 5.0, 4.0),
            CartesianVector::new(3.0, 2.0, 1.0),
        );
        let c = a.multiply_matrix(b);
        assert_eq!(c.row_1().x(), 30.0);
        assert_eq!(c.row_1().y(), 24.0);
        assert_eq!(c.row_1().z(), 18.0);
        assert_eq!(c.row_2().x(), 84.0);
        assert_eq!(c.row_2().y(), 69.0);
        assert_eq!(c.row_2().z(), 54.0);
        assert_eq!(c.row_3().x(), 138.0);
        assert_eq!(c.row_3().y(), 114.0);
        assert_eq!(c.row_3().z(), 90.0);
    }

    #[test]
    fn test_cartesian_matrix_multiply_vector() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = CartesianVector::new(1.0, 2.0, 3.0);
        let c = a.multiply_vector(b);
        assert_eq!(c.x(), 14.0);
        assert_eq!(c.y(), 32.0);
        assert_eq!(c.z(), 50.0);
    }

    #[test]
    fn test_cartesian_matrix_row_1() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.row_1();
        assert_eq!(b.x(), 1.0);
        assert_eq!(b.y(), 2.0);
        assert_eq!(b.z(), 3.0);
    }

    #[test]
    fn test_cartesian_matrix_row_2() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.row_2();
        assert_eq!(b.x(), 4.0);
        assert_eq!(b.y(), 5.0);
        assert_eq!(b.z(), 6.0);
    }

    #[test]
    fn test_cartesian_matrix_row_3() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.row_3();
        assert_eq!(b.x(), 7.0);
        assert_eq!(b.y(), 8.0);
        assert_eq!(b.z(), 9.0);
    }

    #[test]
    fn test_cartesian_matrix_column_1() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.column_1();
        assert_eq!(b.x(), 1.0);
        assert_eq!(b.y(), 4.0);
        assert_eq!(b.z(), 7.0);
    }

    #[test]
    fn test_cartesian_matrix_column_2() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.column_2();
        assert_eq!(b.x(), 2.0);
        assert_eq!(b.y(), 5.0);
        assert_eq!(b.z(), 8.0);
    }

    #[test]
    fn test_cartesian_matrix_column_3() {
        let a = CartesianMatrix::new(
            CartesianVector::new(1.0, 2.0, 3.0),
            CartesianVector::new(4.0, 5.0, 6.0),
            CartesianVector::new(7.0, 8.0, 9.0),
        );
        let b = a.column_3();
        assert_eq!(b.x(), 3.0);
        assert_eq!(b.y(), 6.0);
        assert_eq!(b.z(), 9.0);
    }
}
