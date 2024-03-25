/// Adds two vectors of arbitrary length
/// # Example
/// ```rust
/// use linear_algebra::vectors::functions::add;
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![4.0, 5.0, 6.0];
/// let result = add(a, b);
/// println!("{:?}", result); // [5.0, 7.0, 9.0]
/// ```
pub fn add(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in 0..a.len() {
        result.push(a[i] + b[i]);
    }
    result
}

/// Subtracts two vectors of arbitrary length
/// # Example
/// ```rust
/// use linear_algebra::vectors::functions::subtract;
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![4.0, 5.0, 6.0];
/// let result = subtract(a, b);
/// println!("{:?}", result); // [-3.0, -3.0, -3.0]
/// ```
pub fn subtract(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in 0..a.len() {
        result.push(a[i] - b[i]);
    }
    result
}

/// Calculates the dot product of two vectors of arbitrary length
/// # Example
/// ```rust
/// use linear_algebra::vectors::functions::dot;
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![4.0, 5.0, 6.0];
/// let result = dot(a, b);
/// println!("{:?}", result); // 32.0
/// ```
pub fn dot(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    result
}

/// Scales a vector by a scalar
/// # Example
/// ```rust
/// use linear_algebra::vectors::functions::scale;
/// let a = vec![1.0, 2.0, 3.0];
/// let b = 2.0;
/// let result = scale(a, b);
/// println!("{:?}", result); // [2.0, 4.0, 6.0]
/// ```
pub fn scale(a: Vec<f64>, b: f64) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in 0..a.len() {
        result.push(a[i] * b);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = add(a, b);
        assert_eq!(result, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    pub fn test_subtract() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = subtract(a, b);
        assert_eq!(result, vec![-3.0, -3.0, -3.0]);
    }

    #[test]
    pub fn test_dot() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = dot(a, b);
        assert_eq!(result, 32.0);
    }

    #[test]
    pub fn test_scale() {
        let a = vec![1.0, 2.0, 3.0];
        let b = 2.0;
        let result = scale(a, b);
        assert_eq!(result, vec![2.0, 4.0, 6.0]);
    }
}
