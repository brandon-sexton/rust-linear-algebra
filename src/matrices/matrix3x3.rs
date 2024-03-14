use crate::vectors::vector3d::Vector3D;

/// Represents a 3x3 matrix.
#[derive(Debug, PartialEq)]
pub struct Matrix3x3 {
    rows: [Vector3D; 3],
}

impl Matrix3x3 {
    /// Creates a new 3x3 matrix with the given rows.
    pub fn new(row1: Vector3D, row2: Vector3D, row3: Vector3D) -> Matrix3x3 {
        Matrix3x3 {
            rows: [row1, row2, row3],
        }
    }

    /// Returns the row at the given index.
    pub fn row(&self, index: usize) -> Vector3D {
        self.rows[index]
    }

    /// Returns the column at the given index.
    pub fn column(&self, index: usize) -> Vector3D {
        Vector3D::new(
            self.rows[0].element(index),
            self.rows[1].element(index),
            self.rows[2].element(index),
        )
    }

    /// Calculates the determinant of the matrix.
    pub fn determinant(&self) -> f64 {
        self.rows[0].element(0)
            * (self.rows[1].element(1) * self.rows[2].element(2)
                - self.rows[1].element(2) * self.rows[2].element(1))
            - self.rows[0].element(1)
                * (self.rows[1].element(0) * self.rows[2].element(2)
                    - self.rows[1].element(2) * self.rows[2].element(0))
            + self.rows[0].element(2)
                * (self.rows[1].element(0) * self.rows[2].element(1)
                    - self.rows[1].element(1) * self.rows[2].element(0))
    }

    /// Returns the transpose of the matrix.
    pub fn transpose(&self) -> Matrix3x3 {
        Matrix3x3::new(self.column(0), self.column(1), self.column(2))
    }

    /// Returns the sum of this matrix and another matrix.
    pub fn plus(&self, other: &Matrix3x3) -> Matrix3x3 {
        Matrix3x3::new(
            self.row(0).plus(&other.row(0)),
            self.row(1).plus(&other.row(1)),
            self.row(2).plus(&other.row(2)),
        )
    }

    /// Returns the difference between this matrix and another matrix.
    pub fn minus(&self, other: &Matrix3x3) -> Matrix3x3 {
        Matrix3x3::new(
            self.row(0).minus(&other.row(0)),
            self.row(1).minus(&other.row(1)),
            self.row(2).minus(&other.row(2)),
        )
    }

    /// Returns the product of this matrix and a scalar.
    pub fn times_scalar(&self, scalar: f64) -> Matrix3x3 {
        Matrix3x3::new(
            self.row(0).times(scalar),
            self.row(1).times(scalar),
            self.row(2).times(scalar),
        )
    }

    /// Returns the product of this matrix and another 3x3 matrix.
    pub fn times_matrix3x3(&self, other: &Matrix3x3) -> Matrix3x3 {
        let row1 = Vector3D::new(
            self.row(0).dot(&other.column(0)),
            self.row(0).dot(&other.column(1)),
            self.row(0).dot(&other.column(2)),
        );
        let row2 = Vector3D::new(
            self.row(1).dot(&other.column(0)),
            self.row(1).dot(&other.column(1)),
            self.row(1).dot(&other.column(2)),
        );
        let row3 = Vector3D::new(
            self.row(2).dot(&other.column(0)),
            self.row(2).dot(&other.column(1)),
            self.row(2).dot(&other.column(2)),
        );

        Matrix3x3::new(row1, row2, row3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_new() {
        let row1 = Vector3D::new(1.0, 2.0, 3.0);
        let row2 = Vector3D::new(4.0, 5.0, 6.0);
        let row3 = Vector3D::new(7.0, 8.0, 9.0);
        let matrix = Matrix3x3::new(row1, row2, row3);

        assert_eq!(matrix.row(0), row1);
        assert_eq!(matrix.row(1), row2);
        assert_eq!(matrix.row(2), row3);
    }

    #[test]
    fn test_column() {
        let row1 = Vector3D::new(1.0, 2.0, 3.0);
        let row2 = Vector3D::new(4.0, 5.0, 6.0);
        let row3 = Vector3D::new(7.0, 8.0, 9.0);
        let matrix = Matrix3x3::new(row1, row2, row3);

        assert_eq!(matrix.column(0), Vector3D::new(1.0, 4.0, 7.0));
        assert_eq!(matrix.column(1), Vector3D::new(2.0, 5.0, 8.0));
        assert_eq!(matrix.column(2), Vector3D::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_determinant() {
        let row1 = Vector3D::new(1.0, 2.0, 3.0);
        let row2 = Vector3D::new(4.0, 5.0, 6.0);
        let row3 = Vector3D::new(7.0, 8.0, 9.0);
        let matrix = Matrix3x3::new(row1, row2, row3);

        assert_approx_eq!(matrix.determinant(), 0.0);
    }

    #[test]
    fn test_transpose() {
        let row1 = Vector3D::new(1.0, 2.0, 3.0);
        let row2 = Vector3D::new(4.0, 5.0, 6.0);
        let row3 = Vector3D::new(7.0, 8.0, 9.0);
        let matrix = Matrix3x3::new(row1, row2, row3);

        let expected = Matrix3x3::new(
            Vector3D::new(1.0, 4.0, 7.0),
            Vector3D::new(2.0, 5.0, 8.0),
            Vector3D::new(3.0, 6.0, 9.0),
        );

        assert_eq!(matrix.transpose(), expected);
    }

    #[test]
    fn test_plus() {
        let row1 = Vector3D::new(1.0, 2.0, 3.0);
        let row2 = Vector3D::new(4.0, 5.0, 6.0);
        let row3 = Vector3D::new(7.0, 8.0, 9.0);
        let matrix1 = Matrix3x3::new(row1, row2, row3);

        let row4 = Vector3D::new(10.0, 11.0, 12.0);
        let row5 = Vector3D::new(13.0, 14.0, 15.0);
        let row6 = Vector3D::new(16.0, 17.0, 18.0);
        let matrix2 = Matrix3x3::new(row4, row5, row6);

        let expected = Matrix3x3::new(
            Vector3D::new(11.0, 13.0, 15.0),
            Vector3D::new(17.0, 19.0, 21.0),
            Vector3D::new(23.0, 25.0, 27.0),
        );

        assert_eq!(matrix1.plus(&matrix2), expected);
    }

    #[test]
    fn test_minus() {
        let row1 = Vector3D::new(1.0, 2.0, 3.0);
        let row2 = Vector3D::new(4.0, 5.0, 6.0);
        let row3 = Vector3D::new(7.0, 8.0, 9.0);
        let matrix1 = Matrix3x3::new(row1, row2, row3);

        let row4 = Vector3D::new(10.0, 11.0, 12.0);
        let row5 = Vector3D::new(13.0, 14.0, 15.0);
        let row6 = Vector3D::new(16.0, 17.0, 18.0);
        let matrix2 = Matrix3x3::new(row4, row5, row6);

        let expected = Matrix3x3::new(
            Vector3D::new(-9.0, -9.0, -9.0),
            Vector3D::new(-9.0, -9.0, -9.0),
            Vector3D::new(-9.0, -9.0, -9.0),
        );

        assert_eq!(matrix1.minus(&matrix2), expected);
    }

    #[test]
    fn test_times_scalar() {
        let row1 = Vector3D::new(1.0, 2.0, 3.0);
        let row2 = Vector3D::new(4.0, 5.0, 6.0);
        let row3 = Vector3D::new(7.0, 8.0, 9.0);
        let matrix = Matrix3x3::new(row1, row2, row3);

        let expected = Matrix3x3::new(
            Vector3D::new(2.0, 4.0, 6.0),
            Vector3D::new(8.0, 10.0, 12.0),
            Vector3D::new(14.0, 16.0, 18.0),
        );

        assert_eq!(matrix.times_scalar(2.0), expected);
    }

    #[test]
    fn test_times_matrix3x3() {
        let row1 = Vector3D::new(1.0, 2.0, 3.0);
        let row2 = Vector3D::new(4.0, 5.0, 6.0);
        let row3 = Vector3D::new(7.0, 8.0, 9.0);
        let matrix1 = Matrix3x3::new(row1, row2, row3);

        let row4 = Vector3D::new(10.0, 11.0, 12.0);
        let row5 = Vector3D::new(13.0, 14.0, 15.0);
        let row6 = Vector3D::new(16.0, 17.0, 18.0);
        let matrix2 = Matrix3x3::new(row4, row5, row6);

        let expected = Matrix3x3::new(
            Vector3D::new(84.0, 90.0, 96.0),
            Vector3D::new(201.0, 216.0, 231.0),
            Vector3D::new(318.0, 342.0, 366.0),
        );

        assert_eq!(matrix1.times_matrix3x3(&matrix2), expected);
    }
}
