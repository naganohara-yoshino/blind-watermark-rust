use crate::Block;
use faer::prelude::*;
use std::f32::consts::PI;

impl Block {
    pub fn dct(self) -> Block {
        Block {
            mat_data: dct2_2d(self.mat_data.as_ref()),
        }
    }
    pub fn idct(self) -> Block {
        Block {
            mat_data: dct3_2d(self.mat_data.as_ref()),
        }
    }
}

pub fn dct2_2d(mat: MatRef<f32>) -> Mat<f32> {
    assert!(mat.ncols() == mat.nrows());
    let n = mat.ncols();
    let dct_mat = dct_mat_normalized(n);
    &dct_mat * &mat * &dct_mat.transpose()
}

pub fn dct3_2d(mat: MatRef<f32>) -> Mat<f32> {
    assert!(mat.ncols() == mat.nrows());
    let n = mat.ncols();
    let dct_mat = dct_mat_normalized(n);
    &dct_mat.transpose() * &mat * &dct_mat
}

fn dct_mat_normalized(n: usize) -> Mat<f32> {
    let dct_mat = Mat::from_fn(n, n, |r, c| {
        let i = r as f32;
        let j = c as f32;
        match r {
            0 => f32::cos(PI / (n as f32) * i * (j + 0.5)) * f32::sqrt(1.0 / n as f32),
            _ => f32::cos(PI / (n as f32) * i * (j + 0.5)) * f32::sqrt(2.0 / n as f32),
        }
    });
    dct_mat
}
