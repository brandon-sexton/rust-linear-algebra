use std::vec;

pub fn add(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Vec<f64> {
    match vector1.len() {
        2 => _add_2d(vector1, vector2),
        3 => _add_3d(vector1, vector2),
        _ => panic!("Not implemented"),
    }
}

pub fn subtract(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Vec<f64> {
    match vector1.len() {
        2 => _subtract_2d(vector1, vector2),
        3 => _subtract_3d(vector1, vector2),
        _ => panic!("Not implemented"),
    }
}

pub fn cross(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Vec<f64> {
    match vector1.len() {
        3 => _cross_3d(vector1, vector2),
        _ => panic!("Not implemented"),
    }
}

pub fn dot(vector1: &Vec<f64>, vector2: &Vec<f64>) -> f64 {
    match vector1.len() {
        2 => _dot_2d(vector1, vector2),
        3 => _dot_3d(vector1, vector2),
        _ => panic!("Not implemented"),
    }
}

pub fn scale(vector: &Vec<f64>, scalar: f64) -> Vec<f64> {
    match vector.len() {
        2 => _scale_2d(vector, scalar),
        3 => _scale_3d(vector, scalar),
        _ => panic!("Not implemented"),
    }
}

pub fn magnitude(vector: &Vec<f64>) -> f64 {
    match vector.len() {
        2 => _magnitude_2d(vector),
        3 => _magnitude_3d(vector),
        _ => panic!("Not implemented"),
    }
}

pub fn normalize(vector: &Vec<f64>) -> Vec<f64> {
    match vector.len() {
        2 => _normalize_2d(vector),
        3 => _normalize_3d(vector),
        _ => panic!("Not implemented"),
    }
}

fn _add_2d(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Vec<f64> {
    vec![vector1[0] + vector2[0], vector1[1] + vector2[1]]
}

fn _add_3d(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Vec<f64> {
    vec![
        vector1[0] + vector2[0],
        vector1[1] + vector2[1],
        vector1[2] + vector2[2],
    ]
}

fn _subtract_2d(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Vec<f64> {
    vec![vector1[0] - vector2[0], vector1[1] - vector2[1]]
}

fn _subtract_3d(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Vec<f64> {
    vec![
        vector1[0] - vector2[0],
        vector1[1] - vector2[1],
        vector1[2] - vector2[2],
    ]
}

fn _cross_3d(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Vec<f64> {
    vec![
        vector1[1] * vector2[2] - vector1[2] * vector2[1],
        vector1[2] * vector2[0] - vector1[0] * vector2[2],
        vector1[0] * vector2[1] - vector1[1] * vector2[0],
    ]
}

fn _dot_2d(vector1: &Vec<f64>, vector2: &Vec<f64>) -> f64 {
    vector1[0] * vector2[0] + vector1[1] * vector2[1]
}

fn _dot_3d(vector1: &Vec<f64>, vector2: &Vec<f64>) -> f64 {
    vector1[0] * vector2[0] + vector1[1] * vector2[1] + vector1[2] * vector2[2]
}

fn _scale_2d(vector: &Vec<f64>, scalar: f64) -> Vec<f64> {
    vec![vector[0] * scalar, vector[1] * scalar]
}

fn _scale_3d(vector: &Vec<f64>, scalar: f64) -> Vec<f64> {
    vec![vector[0] * scalar, vector[1] * scalar, vector[2] * scalar]
}

fn _magnitude_2d(vector: &Vec<f64>) -> f64 {
    (vector[0] * vector[0] + vector[1] * vector[1]).sqrt()
}

fn _magnitude_3d(vector: &Vec<f64>) -> f64 {
    (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt()
}

fn _normalize_2d(vector: &Vec<f64>) -> Vec<f64> {
    let magnitude = _magnitude_2d(vector);
    _scale_2d(vector, 1.0 / magnitude)
}

fn _normalize_3d(vector: &Vec<f64>) -> Vec<f64> {
    let magnitude = _magnitude_3d(vector);
    _scale_3d(vector, 1.0 / magnitude)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_2d() {
        let vector1 = vec![1.0, 2.0];
        let vector2 = vec![3.0, 4.0];
        let expected_result = vec![4.0, 6.0];
        assert_eq!(_add_2d(&vector1, &vector2), expected_result);
    }

    #[test]
    fn test_add_3d() {
        let vector1 = vec![1.0, 2.0, 3.0];
        let vector2 = vec![4.0, 5.0, 6.0];
        let expected_result = vec![5.0, 7.0, 9.0];
        assert_eq!(_add_3d(&vector1, &vector2), expected_result);
    }

    #[test]
    #[should_panic(expected = "Not implemented")]
    fn test_add_with_unsupported_dimension() {
        let vector1 = vec![1.0, 2.0, 3.0, 4.0];
        let vector2 = vec![5.0, 6.0, 7.0, 8.0];
        add(&vector1, &vector2);
    }

    #[test]
    fn test_subtract_2d() {
        let vector1 = vec![1.0, 2.0];
        let vector2 = vec![3.0, 4.0];
        let expected_result = vec![-2.0, -2.0];
        assert_eq!(_subtract_2d(&vector1, &vector2), expected_result);
    }

    #[test]
    fn test_subtract_3d() {
        let vector1 = vec![1.0, 2.0, 3.0];
        let vector2 = vec![4.0, 5.0, 6.0];
        let expected_result = vec![-3.0, -3.0, -3.0];
        assert_eq!(_subtract_3d(&vector1, &vector2), expected_result);
    }

    #[test]
    fn test_dot_2d() {
        let vector1 = vec![1.0, 2.0];
        let vector2 = vec![3.0, 4.0];
        let expected_result = 11.0;
        assert_eq!(_dot_2d(&vector1, &vector2), expected_result);
    }

    #[test]
    fn test_dot_3d() {
        let vector1 = vec![1.0, 2.0, 3.0];
        let vector2 = vec![4.0, 5.0, 6.0];
        let expected_result = 32.0;
        assert_eq!(_dot_3d(&vector1, &vector2), expected_result);
    }

    #[test]
    fn test_cross_3d() {
        let vector1 = vec![1.0, 2.0, 3.0];
        let vector2 = vec![4.0, 5.0, 6.0];
        let expected_result = vec![-3.0, 6.0, -3.0];
        assert_eq!(_cross_3d(&vector1, &vector2), expected_result);
    }

    #[test]
    fn test_scale_2d() {
        let vector = vec![1.0, 2.0];
        let scalar = 3.0;
        let expected_result = vec![3.0, 6.0];
        assert_eq!(_scale_2d(&vector, scalar), expected_result);
    }

    #[test]
    fn test_scale_3d() {
        let vector = vec![1.0, 2.0, 3.0];
        let scalar = 3.0;
        let expected_result = vec![3.0, 6.0, 9.0];
        assert_eq!(_scale_3d(&vector, scalar), expected_result);
    }

    #[test]
    fn test_magnitude_2d() {
        let vector = vec![3.0, 4.0];
        let expected_result = 5.0;
        assert_eq!(_magnitude_2d(&vector), expected_result);
    }

    #[test]
    fn test_magnitude_3d() {
        let vector = vec![2.0, 3.0, 6.0];
        let expected_result = 7.0;
        assert_eq!(_magnitude_3d(&vector), expected_result);
    }

    #[test]
    fn test_normalize_2d() {
        let vector = vec![3.0, 4.0];
        let expected_result = vec![0.6, 0.8];
        assert_eq!(_normalize_2d(&vector), expected_result);
    }

    #[test]
    fn test_normalize_3d() {
        let vector = vec![2.0, 3.0, 6.0];
        let expected_result = vec![2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0];
        assert_eq!(_normalize_3d(&vector), expected_result);
    }
}
