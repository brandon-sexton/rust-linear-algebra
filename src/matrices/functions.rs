//!  Module used to perform basic opertations on matrices of arbitrary dimensions
/// Adds two matrices of arbitrary dimensions
/// # Example
/// ## Code
/// ```rust
/// use linear_algebra::matrices::functions::add;
///
/// fn main() {
///     let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
///     let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
///     let result = add(a, b);
///     println!("{:?}", result);
/// }
/// ```
/// ## Terminal
/// ```bash
/// $ cargo run
/// [[6.0, 8.0], [10.0, 12.0]]
/// ```
pub fn add(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for i in 0..a.len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..a[i].len() {
            row.push(a[i][j] + b[i][j]);
        }
        result.push(row);
    }
    result
}

/// Subtracts two matrices of arbitrary dimensions
/// # Example
/// ## Code
/// ```rust
/// use linear_algebra::matrices::functions::subtract;
///
/// fn main() {
///     let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
///     let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
///     let result = subtract(a, b);
///     println!("{:?}", result);
/// }
/// ```
/// ## Terminal
/// ```bash
/// $ cargo run
/// [[-4.0, -4.0], [-4.0, -4.0]]
/// ```
pub fn subtract(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for i in 0..a.len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..a[i].len() {
            row.push(a[i][j] - b[i][j]);
        }
        result.push(row);
    }
    result
}

/// Multiplies two matrices of arbitrary dimensions
/// # Example
/// ## Code
/// ```rust
/// use linear_algebra::matrices::functions::multiply;
///
/// fn main() {
///     let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
///     let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
///     let result = multiply(a, b);
///     println!("{:?}", result);
/// }
/// ```
/// ## Terminal
/// ```bash
/// $ cargo run
/// [[19.0, 22.0], [43.0, 50.0]]
/// ```
pub fn multiply(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for i in 0..a.len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..b[0].len() {
            let mut sum: f64 = 0.0;
            for k in 0..a[i].len() {
                sum += a[i][k] * b[k][j];
            }
            row.push(sum);
        }
        result.push(row);
    }
    result
}

/// Scales a matrix by an input multiple
/// # Example
/// ## Code
/// ```rust
/// use linear_algebra::matrices::functions::scale;
///
/// fn main() {
///     let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
///     let b = 2.0;
///     let result = scale(a, b);
///     println!("{:?}", result);
/// }
/// ```
/// ## Terminal
/// ```bash
/// $ cargo run
/// [[2.0, 4.0], [6.0, 8.0]]
/// ```
pub fn scale(a: Vec<Vec<f64>>, b: f64) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for i in 0..a.len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..a[i].len() {
            row.push(a[i][j] * b);
        }
        result.push(row);
    }
    result
}

/// Gets the transpose of a matrix
/// # Example
/// ## Code
/// ```rust
/// use linear_algebra::matrices::functions::transpose;
///
/// fn main() {
///     let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
///     let result = transpose(a);
///     println!("{:?}", result);
/// }
/// ```
/// ## Terminal
/// ```bash
/// $ cargo run
/// [[1.0, 3.0], [2.0, 4.0]]
/// ```
pub fn transpose(a: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for i in 0..a[0].len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..a.len() {
            row.push(a[j][i]);
        }
        result.push(row);
    }
    result
}

/// Creates an identity matrix of a given size
/// # Example
/// ## Code
/// ```rust
/// use linear_algebra::matrices::functions::identity;
///
/// fn main() {
///     let result = identity(3);
///     println!("{:?}", result);
/// }
/// ```
/// ## Terminal
/// ```bash
/// $ cargo run
/// [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
/// ```
pub fn identity(size: usize) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for i in 0..size {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..size {
            if i == j {
                row.push(1.0);
            } else {
                row.push(0.0);
            }
        }
        result.push(row);
    }
    result
}

/// Calculates the determinant of a square matrix
/// # Example
/// ## Code
/// ```rust
/// use linear_algebra::matrices::functions::determinant;
///
/// fn main() {
///     let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
///     let result = determinant(a);
///     println!("{:?}", result);
/// }
/// ```
/// ## Terminal
/// ```bash
/// $ cargo run
/// -2.0
/// ```
pub fn determinant(a: Vec<Vec<f64>>) -> f64 {
    if a.len() == 2 {
        return a[0][0] * a[1][1] - a[0][1] * a[1][0];
    }
    let mut result: f64 = 0.0;
    for i in 0..a.len() {
        let mut sub_matrix: Vec<Vec<f64>> = Vec::new();
        for j in 1..a.len() {
            let mut row: Vec<f64> = Vec::new();
            for k in 0..a.len() {
                if k != i {
                    row.push(a[j][k]);
                }
            }
            sub_matrix.push(row);
        }
        if i % 2 == 0 {
            result += a[0][i] * determinant(sub_matrix);
        } else {
            result -= a[0][i] * determinant(sub_matrix);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let result = add(a, b);
        assert_eq!(result, vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
    }

    #[test]
    fn test_subtract() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let result = subtract(a, b);
        assert_eq!(result, vec![vec![-4.0, -4.0], vec![-4.0, -4.0]]);
    }

    #[test]
    fn test_multiply() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let result = multiply(a, b);
        assert_eq!(result, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
    }

    #[test]
    fn test_scale() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let result = scale(a, 2.0);
        assert_eq!(result, vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
    }

    #[test]
    fn test_transpose() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let result = transpose(a);
        assert_eq!(result, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
    }

    #[test]
    fn test_identity() {
        let result = identity(3);
        assert_eq!(
            result,
            vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0]
            ]
        );
    }

    #[test]
    fn test_determinant() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let result = determinant(a);
        assert_eq!(result, -2.0);
    }
}
