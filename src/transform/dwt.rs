use faer::Mat;
use std::f64::consts::SQRT_2;

/// 1D Haar DWT on a signal slice (symmetric padding if length is odd).
/// Returns (low_coeffs, high_coeffs).
fn dwt_1d_haar(signal: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = signal.len();
    // If odd, mirror-pad last element (simple symmetric)
    // let has_odd = n % 2 == 1;
    let pairs = (n + 1) / 2;

    let mut low = Vec::with_capacity(pairs);
    let mut high = Vec::with_capacity(pairs);

    for i in 0..pairs {
        let i0 = 2 * i;
        let x0 = signal[i0];
        let x1 = if i0 + 1 < n { signal[i0 + 1] } else { signal[n - 1] }; // symmetric pad
        let l = (x0 + x1) / SQRT_2;
        let h = (x0 - x1) / SQRT_2;
        low.push(l);
        high.push(h);
    }
    (low, high)
}

/// 1D inverse Haar: given low and high, reconstruct signal
fn idwt_1d_haar(low: &[f64], high: &[f64]) -> Vec<f64> {
    let pairs = low.len();
    let mut signal = Vec::with_capacity(pairs * 2);
    for i in 0..pairs {
        let l = low[i];
        let h = high[i];
        let x0 = (l + h) / SQRT_2;
        let x1 = (l - h) / SQRT_2;
        signal.push(x0);
        signal.push(x1);
    }
    signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwt_idwt_1d() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (low, high) = dwt_1d_haar(&signal);
        let reconstructed = idwt_1d_haar(&low, &high);
        // Since we did symmetric padding, the last element is duplicated
        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.0];
        for (a, b) in reconstructed.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }
}


/// 2D single-level Haar DWT using faer_core::Mat<f64>.
/// Returns (cA, cH, cV, cD)
pub fn dwt2_haar(mat: &Mat<f64>) -> (Mat<f64>, Mat<f64>, Mat<f64>, Mat<f64>) {
    let nrows = mat.nrows();
    let ncols = mat.ncols();

    // --- Step 1: row-wise transform ---
    // For each row produce low_row (len = ceil(ncols/2)) and high_row (len = ceil(ncols/2))
    let row_half = (ncols + 1) / 2;
    // store intermediate: low_rows and high_rows as Vec<Vec<f64>>
    let mut low_rows: Vec<Vec<f64>> = Vec::with_capacity(nrows);
    let mut high_rows: Vec<Vec<f64>> = Vec::with_capacity(nrows);

    let view = mat.as_ref();
    for r in 0..nrows {
        // read the row into a Vec<f64>
        let mut row = Vec::with_capacity(ncols);
        for c in 0..ncols {
            // safe read via get/read_unchecked: use safe API if you prefer
            // Using safe get to avoid unsafe in example
            let val = view.get((r..=r, c..=c)).read().into_inner()[0]; // slightly awkward but avoids unsafe
            // The snippet above uses MatRef indexing; if your faer version supports simpler get((r,c)).read(), adapt.
            // For clarity and portability below we'll fallback to reading via the documented read_unchecked (unsafe).
            row.push(val);
        }
        let (l, h) = dwt_1d_haar(&row);
        low_rows.push(l);
        high_rows.push(h);
    }

    // --- Step 2: build intermediate matrices (low_part and high_part) with dims (nrows, row_half) each ---
    let mut low_part = Mat::from_fn(nrows, row_half, |r, c| low_rows[r][c]);
    let mut high_part = Mat::from_fn(nrows, row_half, |r, c| high_rows[r][c]);

    // --- Step 3: column-wise transform on low_part and high_part to produce final subbands ---
    // For each column index in 0..row_half, collect column vector and apply dwt_1d_haar
    let col_half = (nrows + 1) / 2;

    // prepare storage for resulting subbands: each subband size = (col_half, row_half)
    let mut cA = Mat::zeros(col_half, row_half); // LL -> approximation
    let mut cV = Mat::zeros(col_half, row_half); // LH -> vertical detail
    let mut cH = Mat::zeros(col_half, row_half); // HL -> horizontal detail
    let mut cD = Mat::zeros(col_half, row_half); // HH -> diagonal detail

    let low_view = low_part.as_ref();
    let high_view = high_part.as_ref();

    for c in 0..row_half {
        // collect low column
        let mut col_low = Vec::with_capacity(nrows);
        let mut col_high = Vec::with_capacity(nrows);
        for r in 0..nrows {
            // same remark about reads; using a simple (and safe) pattern is recommended for production
            let vlow = low_view.get((r..=r, c..=c)).read().into_inner()[0];
            let vhigh = high_view.get((r..=r, c..=c)).read().into_inner()[0];
            col_low.push(vlow);
            col_high.push(vhigh);
        }
        let (ll_col, lh_col) = dwt_1d_haar(&col_low); // ll_col -> cA column; lh_col -> cV column
        let (hl_col, hh_col) = dwt_1d_haar(&col_high); // hl_col -> cH column; hh_col -> cD column

        // write these into matrices (as columns)
        for r in 0..col_half {
            // set element (r, c)
            // Mat::from_fn / Mat::zeros returns owned Mat; there is no direct set method in docs,
            // but we can mutate via as_ptr_mut() or use Mat::from_fn to reconstruct.
            // For simplicity, create small temporary column-matrices and then assemble -- but that's slow.
            // Instead, we'll use a safe approach: create vectors for each subband and then Mat::from_fn at end.

            // We'll collect columns in temporary 2D vecs (below). For clarity, move write to temp arrays.
        }

        // Instead of writing directly, store columns in temporary arrays:
        // (we'll implement that outside this loop)
    }

    // --- To keep implementation readable and robust, rebuild final subbands from collected column vectors --- 
    // (We redo the column loop but this time store into temp Vec<Vec<f64>>)
    let mut cA_cols: Vec<Vec<f64>> = vec![Vec::with_capacity(col_half); row_half];
    let mut cV_cols: Vec<Vec<f64>> = vec![Vec::with_capacity(col_half); row_half];
    let mut cH_cols: Vec<Vec<f64>> = vec![Vec::with_capacity(col_half); row_half];
    let mut cD_cols: Vec<Vec<f64>> = vec![Vec::with_capacity(col_half); row_half];

    for c in 0..row_half {
        let mut col_low = Vec::with_capacity(nrows);
        let mut col_high = Vec::with_capacity(nrows);
        for r in 0..nrows {
            let vlow = low_view.get((r..=r, c..=c)).read().into_inner()[0];
            let vhigh = high_view.get((r..=r, c..=c)).read().into_inner()[0];
            col_low.push(vlow);
            col_high.push(vhigh);
        }
        let (ll_col, lh_col) = dwt_1d_haar(&col_low);
        let (hl_col, hh_col) = dwt_1d_haar(&col_high);

        // push into column containers
        for r in 0..col_half {
            cA_cols[c].push(ll_col[r]);
            cV_cols[c].push(lh_col[r]);
            cH_cols[c].push(hl_col[r]);
            cD_cols[c].push(hh_col[r]);
        }
    }

    // Now build Mat from column-major data: Mat::from_fn expects (row, col)
    let cA = Mat::from_fn(col_half, row_half, |r, c| cA_cols[c][r]);
    let cV = Mat::from_fn(col_half, row_half, |r, c| cV_cols[c][r]);
    let cH = Mat::from_fn(col_half, row_half, |r, c| cH_cols[c][r]);
    let cD = Mat::from_fn(col_half, row_half, |r, c| cD_cols[c][r]);

    (cA, cH, cV, cD)
}

/// 2D inverse single-level Haar IDWT: takes (cA, cH, cV, cD) and reconstructs matrix.
/// Assumes each coefficient Mat has same shape (r_half, c_half). Reconstructs (2*r_half, 2*c_half) (or with padding)
pub fn idwt2_haar(cA: &Mat<f64>, cH: &Mat<f64>, cV: &Mat<f64>, cD: &Mat<f64>) -> Mat<f64> {
    let r_half = cA.nrows();
    let c_half = cA.ncols();

    // First: for each column index in 0..c_half, reconstruct the low_col and high_col of length 2*r_half
    // Then for each reconstructed row, perform inverse along rows.

    // Build low_part and high_part matrices of shape (2*r_half, c_half)
    let mut low_part_cols: Vec<Vec<f64>> = vec![Vec::with_capacity(2 * r_half); c_half];
    let mut high_part_cols: Vec<Vec<f64>> = vec![Vec::with_capacity(2 * r_half); c_half];

    let a_view = cA.as_ref();
    let h_view = cH.as_ref();
    let v_view = cV.as_ref();
    let d_view = cD.as_ref();

    for c in 0..c_half {
        // build low-col (from cA and cV): idwt_1d_haar(low=col_of_cA, high=col_of_cV)
        let mut col_a = Vec::with_capacity(r_half);
        let mut col_v = Vec::with_capacity(r_half);
        let mut col_h = Vec::with_capacity(r_half);
        let mut col_d = Vec::with_capacity(r_half);
        for r in 0..r_half {
            let va = a_view.get((r..=r, c..=c)).read().into_inner()[0];
            let vv = v_view.get((r..=r, c..=c)).read().into_inner()[0];
            let vh = h_view.get((r..=r, c..=c)).read().into_inner()[0];
            let vd = d_view.get((r..=r, c..=c)).read().into_inner()[0];
            col_a.push(va);
            col_v.push(vv);
            col_h.push(vh);
            col_d.push(vd);
        }
        let low_col = idwt_1d_haar(&col_a, &col_v); // length = 2*r_half
        let high_col = idwt_1d_haar(&col_h, &col_d);
        low_part_cols[c] = low_col;
        high_part_cols[c] = high_col;
    }

    // Now low_part and high_part are matrices of shape (2*r_half, c_half) stored as columns
    let nrows = 2 * r_half;
    let ncols = c_half * 2; // after next step we expand horizontally
    // Next: for each row i in 0..nrows, reconstruct full row by idwt_1d_haar on corresponding low and high row vectors
    let mut full_rows: Vec<Vec<f64>> = Vec::with_capacity(nrows);
    for r in 0..nrows {
        // build low_row (length c_half) from low_part_cols[*][r]
        let mut low_row = Vec::with_capacity(c_half);
        let mut high_row = Vec::with_capacity(c_half);
        for c in 0..c_half {
            low_row.push(low_part_cols[c][r]);
            high_row.push(high_part_cols[c][r]);
        }
        let row = idwt_1d_haar(&low_row, &high_row); // length 2*c_half
        full_rows.push(row);
    }

    // Build final Mat from rows
    let mat = Mat::from_fn(nrows, 2 * c_half, |r, c| full_rows[r][c]);
    mat
}
