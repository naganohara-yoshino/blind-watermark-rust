use faer::prelude::*;
use std::f32::consts::PI;

/// Perform 2D Type-II Discrete Cosine Transform (DCT-II)
pub fn dct2_2d(mat: MatRef<f32>) -> Mat<f32> {
    assert!(mat.ncols() == mat.nrows());
    let n = mat.ncols();
    let dct_mat = dct_mat_normalized(n);
    &dct_mat * mat * dct_mat.transpose()
}

/// Perform 2D Type-III Discrete Cosine Transform (DCT-III), also known as IDCT
pub fn dct3_2d(mat: MatRef<f32>) -> Mat<f32> {
    assert!(mat.ncols() == mat.nrows());
    let n = mat.ncols();
    let dct_mat = dct_mat_normalized(n);
    dct_mat.transpose() * mat * dct_mat
}

/// Generate a normalized DCT matrix of size n x n
fn dct_mat_normalized(n: usize) -> Mat<f32> {
    Mat::from_fn(n, n, |r, c| {
        let i = r as f32;
        let j = c as f32;
        match r {
            0 => f32::cos(PI / (n as f32) * i * (j + 0.5)) * f32::sqrt(1.0 / n as f32),
            _ => f32::cos(PI / (n as f32) * i * (j + 0.5)) * f32::sqrt(2.0 / n as f32),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_dct2_2d() {
        let mat = mat![[1.0, 2.0], [3.0, 4.0],];
        let out = dct2_2d(mat.as_ref());
        let expected = mat![[5.0, -1.0], [-2.0, 0.0],];
        for r in 0..out.nrows() {
            for c in 0..out.ncols() {
                assert_relative_eq!(out[(r, c)], expected[(r, c)], epsilon = 1e-6);
            }
        }
    }
    #[test]
    fn test_dct3_2d() {
        let mat = mat![[5.0, -1.0], [-2.0, 0.0],];
        let out = dct3_2d(mat.as_ref());
        let expected = mat![[1.0, 2.0], [3.0, 4.0],];
        for r in 0..out.nrows() {
            for c in 0..out.ncols() {
                assert_relative_eq!(out[(r, c)], expected[(r, c)], epsilon = 1e-6);
            }
        }
    }
}
