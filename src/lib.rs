use faer::prelude::*;

pub mod config;
pub mod strategy;
pub mod transform;

pub struct YCrBrAMat {
    pub y: Mat<f32>,
    pub cb: Mat<f32>,
    pub cr: Mat<f32>,
    pub a: Mat<f32>,
    /// (height, width),
    pub dimensions: (usize, usize),
}

pub struct PaddedYCrBrAMat {
    pub y: Mat<f32>,
    pub cb: Mat<f32>,
    pub cr: Mat<f32>,
    pub a: Mat<f32>,
    pub original_dimensions: (usize, usize),
}
