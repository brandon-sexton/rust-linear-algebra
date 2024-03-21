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

pub fn clean_print(a: Vec<Vec<f64>>, precision: usize) {
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            print!("{:.precision$} ", a[i][j], precision = precision);
        }
        println!();
    }
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

    #[test]
    fn test_clean_print() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        clean_print(a, 2);
    }
}
