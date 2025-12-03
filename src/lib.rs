//! io related types from image crate <-> YCrBrAMat and  YCrBrAMat <-> PaddedYCrBrAMat  is done by simple `into()`
//! The transform pipeline is as follows (io ignored):
//! Embed : PaddedYCrBrAMat -(dwt)->DwtedYCrBrAMat -(cut)-> BlockCutted -(black embed)-> Imbedded -(assemble)-> AssembledYCrBrAMat -(dedwt)-> PaddedYCrBrAMat
//! Extract : PaddedYCrBrAMat -(dwt)->DwtedYCrBrAMat -(cut)-> BlockCutted， and we can read from Blocks
use faer::prelude::*;

pub mod config;
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
    /// (height, width),
    pub original_dimensions: (usize, usize),
}

pub struct DwtedYCrBrAMat {
    ///(LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    ///(LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    ///(LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    pub a: Mat<f32>,
    /// (height, width),
    pub original_dimensions: (usize, usize),
}

pub struct BlockCutted {
    // Take LL part only
    pub y_ll_blocks: Vec<Block>,
    pub cb_ll_blocks: Vec<Block>,
    pub cr_ll_blocks: Vec<Block>,

    //Preserved for recovery
    ///(LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    ///(LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    ///(LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    pub a: Mat<f32>,
    /// (height, width),
    pub original_dimensions: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct Block {
    /// 4×4 block matrix
    pub mat_data: Mat<f32>,
    // index in some serial, for strategy purpose
    //pub index: usize,
}

pub struct Imbedded {
    // Take LL part only
    pub y_ll_blocks: Vec<Block>,
    pub cb_ll_blocks: Vec<Block>,
    pub cr_ll_blocks: Vec<Block>,

    //Preserved for recovery
    ///(LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    ///(LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    ///(LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    pub a: Mat<f32>,
    /// (height, width),
    pub original_dimensions: (usize, usize),
}

pub struct AssembledYCrBrAMat {
    ///(LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    ///(LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    ///(LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    pub a: Mat<f32>,
    /// (height, width),
    pub original_dimensions: (usize, usize),
}
