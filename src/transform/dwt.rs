use faer::traits::ComplexField;
use faer::{Mat, MatRef};
use num::Float;

/// 单层二维 Haar DWT：直接 2x2 block，返回 (LL, HL, LH, HH)
fn haar_dwt2<T: Float + ComplexField>(mat: MatRef<T>) -> (Mat<T>, Mat<T>, Mat<T>, Mat<T>) {
    let (rows, cols) = mat.shape();
    assert!(rows % 2 == 0 && cols % 2 == 0);

    let h = rows / 2;
    let w = cols / 2;

    let mut ll = Mat::zeros(h, w);
    let mut hl = Mat::zeros(h, w);
    let mut lh = Mat::zeros(h, w);
    let mut hh = Mat::zeros(h, w);

    let two = T::from(2.0).unwrap();

    for r in 0..h {
        for c in 0..w {
            let a = mat[(2 * r, 2 * c)];
            let b = mat[(2 * r, 2 * c + 1)];
            let d = mat[(2 * r + 1, 2 * c)];
            let e = mat[(2 * r + 1, 2 * c + 1)];

            ll[(r, c)] = (a + b + d + e) / two;
            hl[(r, c)] = (a - b + d - e) / two;
            lh[(r, c)] = (a + b - d - e) / two;
            hh[(r, c)] = (a - b - d + e) / two;
        }
    }

    (ll, hl, lh, hh)
}

/// 逆变换：由 (LL, HL, LH, HH) 重建原矩阵
fn haar_idwt2<T: Float + ComplexField>(
    ll: MatRef<T>,
    hl: MatRef<T>,
    lh: MatRef<T>,
    hh: MatRef<T>,
) -> Mat<T> {
    let (h, w) = ll.shape();
    let rows = h * 2;
    let cols = w * 2;

    let mut out = Mat::zeros(rows, cols);
    let two = T::from(2.0).unwrap();

    for r in 0..h {
        for c in 0..w {
            let llc = ll[(r, c)];
            let hlc = hl[(r, c)];
            let lhc = lh[(r, c)];
            let hhc = hh[(r, c)];

            let a = (llc + hlc + lhc + hhc) / two;
            let b = (llc - hlc + lhc - hhc) / two;
            let d = (llc + hlc - lhc - hhc) / two;
            let e = (llc - hlc - lhc + hhc) / two;

            out[(2 * r, 2 * c)] = a;
            out[(2 * r, 2 * c + 1)] = b;
            out[(2 * r + 1, 2 * c)] = d;
            out[(2 * r + 1, 2 * c + 1)] = e;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use faer::prelude::*;

    #[test]
    fn test_haar_dwt_2d_f64() {
        let data = mat![
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        // 直接二维 Haar 分解
        let (ll, hl, lh, hh) = haar_dwt2(data.as_ref());
        let expected_ll = mat![[7.0, 11.0], [23.0, 27.0],];
        let expected_hl = Mat::<f64>::ones(2, 2) * (-1.0);
        let expected_lh = Mat::<f64>::ones(2, 2) * (-4.0);
        let expected_hh = Mat::<f64>::zeros(2, 2);

        for r in 0..2 {
            for c in 0..2 {
                assert_relative_eq!(ll[(r, c)], expected_ll[(r, c)], epsilon = 1e-6);
                assert_relative_eq!(hl[(r, c)], expected_hl[(r, c)], epsilon = 1e-6);
                assert_relative_eq!(lh[(r, c)], expected_lh[(r, c)], epsilon = 1e-6);
                assert_relative_eq!(hh[(r, c)], expected_hh[(r, c)], epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_haar_idwt_2d_f64() {
        let ll = mat![[7.0, 11.0], [23.0, 27.0],];
        let hl = Mat::<f64>::ones(2, 2) * (-1.0);
        let lh = Mat::<f64>::ones(2, 2) * (-4.0);
        let hh = Mat::<f64>::zeros(2, 2);

        let reconstructed = haar_idwt2(ll.as_ref(), hl.as_ref(), lh.as_ref(), hh.as_ref());
        let expected = mat![
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        for r in 0..4 {
            for c in 0..4 {
                assert_relative_eq!(reconstructed[(r, c)], expected[(r, c)], epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_haar_dwt_2d_f32() {
        let data = mat![
            [1.0f32, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        // 直接二维 Haar 分解
        let (ll, hl, lh, hh) = haar_dwt2(data.as_ref());
        let expected_ll = mat![[7.0f32, 11.0], [23.0, 27.0],];
        let expected_hl = Mat::<f32>::ones(2, 2) * (-1.0);
        let expected_lh = Mat::<f32>::ones(2, 2) * (-4.0);
        let expected_hh = Mat::<f32>::zeros(2, 2);

        for r in 0..2 {
            for c in 0..2 {
                assert_relative_eq!(ll[(r, c)], expected_ll[(r, c)], epsilon = 1e-6);
                assert_relative_eq!(hl[(r, c)], expected_hl[(r, c)], epsilon = 1e-6);
                assert_relative_eq!(lh[(r, c)], expected_lh[(r, c)], epsilon = 1e-6);
                assert_relative_eq!(hh[(r, c)], expected_hh[(r, c)], epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_haar_idwt_2d_f32() {
        let ll = mat![[7.0f32, 11.0], [23.0, 27.0],];
        let hl = Mat::<f32>::ones(2, 2) * (-1.0);
        let lh = Mat::<f32>::ones(2, 2) * (-4.0);
        let hh = Mat::<f32>::zeros(2, 2);

        let reconstructed = haar_idwt2(ll.as_ref(), hl.as_ref(), lh.as_ref(), hh.as_ref());
        let expected = mat![
            [1.0f32, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        for r in 0..4 {
            for c in 0..4 {
                assert_relative_eq!(reconstructed[(r, c)], expected[(r, c)], epsilon = 1e-6);
            }
        }
    }
}
