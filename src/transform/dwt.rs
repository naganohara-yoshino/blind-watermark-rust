use faer::{Mat, MatRef};

/// 单层二维 Haar DWT：直接 2x2 block，返回 (LL, HL, LH, HH)
fn haar_dwt2(mat: MatRef<f64>) -> (Mat<f64>, Mat<f64>, Mat<f64>, Mat<f64>) {
    let (rows, cols) = mat.shape();
    assert!(rows % 2 == 0 && cols % 2 == 0);

    let h = rows / 2;
    let w = cols / 2;

    let mut ll = Mat::zeros(h, w);
    let mut hl = Mat::zeros(h, w);
    let mut lh = Mat::zeros(h, w);
    let mut hh = Mat::zeros(h, w);

    for r in 0..h {
        for c in 0..w {
            let a = mat[(2 * r, 2 * c)];
            let b = mat[(2 * r, 2 * c + 1)];
            let d = mat[(2 * r + 1, 2 * c)];
            let e = mat[(2 * r + 1, 2 * c + 1)];

            ll[(r, c)] = (a + b + d + e) / 2.0;
            hl[(r, c)] = (a - b + d - e) / 2.0;
            lh[(r, c)] = (a + b - d - e) / 2.0;
            hh[(r, c)] = (a - b - d + e) / 2.0;
        }
    }

    (ll, hl, lh, hh)
}

/// 逆变换：由 (LL, HL, LH, HH) 重建原矩阵
fn haar_idwt2(ll: MatRef<f64>, hl: MatRef<f64>, lh: MatRef<f64>, hh: MatRef<f64>) -> Mat<f64> {
    let (h, w) = ll.shape();
    let rows = h * 2;
    let cols = w * 2;

    let mut out = Mat::zeros(rows, cols);

    for r in 0..h {
        for c in 0..w {
            let llc = ll[(r, c)];
            let hlc = hl[(r, c)];
            let lhc = lh[(r, c)];
            let hhc = hh[(r, c)];

            let a = (llc + hlc + lhc + hhc) / 2.0;
            let b = (llc - hlc + lhc - hhc) / 2.0;
            let d = (llc + hlc - lhc - hhc) / 2.0;
            let e = (llc - hlc - lhc + hhc) / 2.0;

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
    use faer::prelude::*;

    #[test]
    fn test_haar_dwt_2d() {
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
                assert!((ll[(r, c)] - expected_ll[(r, c)]).abs() < 1e-6);
                assert!((hl[(r, c)] - expected_hl[(r, c)]).abs() < 1e-6);
                assert!((lh[(r, c)] - expected_lh[(r, c)]).abs() < 1e-6);
                assert!((hh[(r, c)] - expected_hh[(r, c)]).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn test_haar_idwt_2d() {
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
                assert!((reconstructed[(r, c)] - expected[(r, c)]).abs() < 1e-6);
            }
        }
    }
}
