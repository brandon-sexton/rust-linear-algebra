pub fn add(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in 0..a.len() {
        result.push(a[i] + b[i]);
    }
    result
}

pub fn subtract(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in 0..a.len() {
        result.push(a[i] - b[i]);
    }
    result
}

pub fn dot(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    result
}

pub fn scale(a: Vec<f64>, b: f64) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in 0..a.len() {
        result.push(a[i] * b);
    }
    result
}

pub fn cross(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    result.push(a[1] * b[2] - a[2] * b[1]);
    result.push(a[2] * b[0] - a[0] * b[2]);
    result.push(a[0] * b[1] - a[1] * b[0]);
    result
}

pub fn magnitude(a: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..a.len() {
        result += a[i] * a[i];
    }
    result.sqrt()
}

pub fn normalize(a: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    let mag = magnitude(a.clone());
    for i in 0..a.len() {
        result.push(a[i] / mag);
    }
    result
}

pub fn clean_print(a: Vec<f64>, precision: usize) {
    for i in 0..a.len() {
        println!("{:.precision$}", a[i], precision = precision);
    }
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

    #[test]
    pub fn test_cross() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = cross(a, b);
        assert_eq!(result, vec![-3.0, 6.0, -3.0]);
    }

    #[test]
    pub fn test_magnitude() {
        let a = vec![1.0, 2.0, 3.0];
        let result = magnitude(a);
        assert_eq!(result, 3.7416573867739413);
    }

    #[test]
    pub fn test_normalize() {
        let a = vec![1.0, 2.0, 3.0];
        let result = normalize(a);
        assert_eq!(
            result,
            vec![0.2672612419124244, 0.5345224838248488, 0.8017837257372732]
        );
    }

    #[test]
    pub fn test_clean_print() {
        let a = vec![1.0, 2.0, 3.0];
        let precision = 2;
        clean_print(a, precision);
    }
}
