use faer::prelude::*;
use faer::{Col, ColRef, Mat, MatMut, MatRef};
use std::f64::consts::SQRT_2;

const INV_SQRT_2: f64 = 1.0 / SQRT_2;

/// 一维 Haar DWT
pub fn haar_dwt(input: ColRef<f64>) -> Col<f64> {
    let n = input.nrows();
    assert!(n % 2 == 0);

    let half = n / 2;
    let mut out = Col::<f64>::zeros(n);

    for i in 0..half {
        let x0 = input[i];
        let x1 = input[i + half]; // 注意后续填充位置
        out[i] = (input[2 * i] + input[2 * i + 1]) * INV_SQRT_2;
        out[i + half] = (input[2 * i] - input[2 * i + 1]) * INV_SQRT_2;
    }
    out
}

/// 一维 Haar 逆变换
pub fn haar_idwt(input: ColRef<f64>) -> Col<f64> {
    let n = input.nrows();
    assert!(n % 2 == 0);
    let half = n / 2;

    let mut out = Col::<f64>::zeros(n);

    for i in 0..half {
        let a = input[i];
        let d = input[i + half];
        out[2 * i] = (a + d) * INV_SQRT_2;
        out[2 * i + 1] = (a - d) * INV_SQRT_2;
    }
    out
}


/// 单层二维 Haar DWT：直接 2x2 block，返回 (LL, HL, LH, HH)
fn haar_dwt2(mat: &Mat<f64>) -> (Mat<f64>, Mat<f64>, Mat<f64>, Mat<f64>) {
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
fn haar_idwt2d_blocks(ll: &Mat<f64>, hl: &Mat<f64>, lh: &Mat<f64>, hh: &Mat<f64>) -> Mat<f64> {
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

    #[test]
    fn test_haar_dwt_1d() {
        let col = col![1.0, 2.0, 3.0, 4.0];
        let transformed = haar_dwt(col.as_ref());
        let expected = col![
            SQRT_2 * (10.0),
            SQRT_2 * (22.0),
            SQRT_2 * (-2.0),
            SQRT_2 * (-2.0),
        ];
        for i in 0..transformed.nrows() {
            assert!((transformed[i] - expected[i]).abs() < 1e-6);
        }
    }

    #[test]
    fn test_haar_dwt_2d() {
        let data = mat![
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        // 直接二维 Haar 分解
        let (ll, hl, lh, hh) = haar_dwt2(&data);
        let expected_ll = mat![
            [7.0, 11.0],
            [23.0, 27.0],
        ];
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
        let ll = mat![
            [7.0, 11.0],
            [23.0, 27.0],
        ];
        let hl = Mat::<f64>::ones(2, 2) * (-1.0);
        let lh = Mat::<f64>::ones(2, 2) * (-4.0);
        let hh = Mat::<f64>::zeros(2, 2);

        let reconstructed = haar_idwt2d_blocks(&ll, &hl, &lh, &hh);
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
