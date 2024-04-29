/// Gaussian Elimination of Quadratic Matrices
/// Takes an augmented matrix as input, returns vector of results
/// solves a*x = b where `a` is a square matrix and `b` is a vector
/// # Arguments
/// * `a` - matrix `a` in a*x = b
/// * `b` - vector `b` in a*x = b
/// # Returns
/// * `Option<Vec<f64>>` - vector of results
use ordered_float::OrderedFloat;
use rayon::prelude::*;

pub fn gaussian_elimination_serial(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    if a.is_empty() || a.len() != a[0].len() || a.len() != b.len() {
        return None;
    }
    let n = a.len();
    let mut mat = prepare(a, b);
    for col in 0..n {
        let row = (col..n)
            .max_by_key(|&i| OrderedFloat(mat[i][col].abs()))
            .unwrap();
        if mat[row][col].abs() < f64::EPSILON {
            return None;
        }
        if row != col {
            mat.swap(row, col);
        }
        let pivot = mat[col][col];
        for i in 0..n {
            if i != col {
                let c = mat[i][col] / pivot;
                for j in 0..=n {
                    mat[i][j] -= mat[col][j] * c;
                }
            }
        }
    }

    let res = mat
        .iter()
        .enumerate()
        .map(|(i, row)| row[n] / row[i])
        .collect();
    Some(res)
}

pub fn gaussian_elimination_par(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    if a.is_empty() || a.len() != a[0].len() || a.len() != b.len() {
        return None;
    }
    let n = a.len();
    let mut mat = prepare(a, b);
    for col in 0..n {
        let row = (col..n)
            .max_by_key(|&i| OrderedFloat(mat[i][col].abs()))
            .unwrap();
        if mat[row][col].abs() < f64::EPSILON {
            return None;
        }
        if row != col {
            mat.swap(row, col);
        }
        let pivot = mat[col][col];
        let (before, after_inclusive) = mat.split_at_mut(col);
        let (row_values, after) = after_inclusive.split_first_mut().unwrap();
        let others = before.par_iter_mut().chain(after.par_iter_mut());
        others.for_each(|row| {
            let c = row[col] / pivot;
            for j in 0..=n {
                row[j] -= row_values[j] * c;
            }
        });
    }

    let res = mat
        .iter()
        .enumerate()
        .map(|(i, row)| row[n] / row[i])
        .collect();
    Some(res)
}

/// prepares the matrix for Gaussian elimination by converting it to an augmented matrix
/// # Arguments
/// * `a` - matrix `a` in a*x = b
/// * `b` - vector `b` in a*x = b
/// # Returns
/// * `Vec<Vec<f64>>` - augmented matrix
fn prepare(a: &[Vec<f64>], b: &[f64]) -> Vec<Vec<f64>> {
    let n = a.len();
    let m = a[0].len();
    let mut mat = vec![vec![0.0; m + 1]; n];
    for i in 0..n {
        for j in 0..m {
            mat[i][j] = a[i][j];
        }
        mat[i][m] = b[i];
    }
    mat
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const A: [[f64; 6]; 6] = [
        [1.5, 2.0, 1.0, -1.0, -2.0, 1.0],
        [3.0, 3.0, -1.0, 16.0, 18.0, 1.0],
        [1.0, 1.0, 3.0, -2.0, -6.0, 1.0],
        [1.0, 1.0, 99.0, 19.0, 2.0, 1.0],
        [1.0, -2.0, 16.0, 1.0, 9.0, 10.0],
        [1.0, 3.0, 1.0, -5.0, 1.0, 1.0],
    ];

    const B: [f64; 6] = [1.0, 1.0, 1.0, 1.0, 1.0, 95.0];

    const EXPECTED: [f64; 6] = [
        -264.0590678357174,
        159.63207198892468,
        -6.15692201199815,
        35.31038301799723,
        -18.806691278264857,
        81.67838024919237,
    ];

    #[test]
    fn test_gauss_serial() {
        let a = A.iter().map(|r| r.to_vec()).collect_vec();
        let result = gaussian_elimination_serial(&a, &B).unwrap();
        assert_eq!(result.len(), EXPECTED.len());
        for (r, e) in result.iter().zip(EXPECTED.iter()) {
            assert!((r - e).abs() < f64::EPSILON, "{} != {}", r, e);
        }
    }

    #[test]
    fn test_gauss_serial_singular() {
        let a: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let b = vec![1.0, 2.0];
        assert_eq!(gaussian_elimination_serial(&a, &b), None);
    }

    #[test]
    fn test_gauss_par() {
        let a = A.iter().map(|r| r.to_vec()).collect_vec();
        let result = gaussian_elimination_par(&a, &B).unwrap();
        assert_eq!(result.len(), EXPECTED.len());
        for (r, e) in result.iter().zip(EXPECTED.iter()) {
            assert!((r - e).abs() < f64::EPSILON, "{} != {}", r, e);
        }
    }

    #[test]
    fn test_gauss_par_singular() {
        let a: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let b = vec![1.0, 2.0];
        assert_eq!(gaussian_elimination_par(&a, &b), None);
    }
}
