//! io related types from image crate <-> YCrBrAMat and  YCrBrAMat <-> PaddedYCrBrAMat  is done by simple `into()`
//! The transform pipeline is as follows (io ignored):
//! Embed : PaddedYCrBrAMat -(dwt)->DwtedYCrBrAMat -(cut)-> BlockCutted -(black embed)-> Imbedded -(assemble)-> AssembledYCrBrAMat -(idwt)-> PaddedYCrBrAMat
//! Extract : PaddedYCrBrAMat -(dwt)->DwtedYCrBrAMat -(cut)-> BlockCutted, and we can read from Blocks

pub mod config;
pub mod prelude;
pub(crate) mod quantization;
pub mod strategy;
pub mod transform;
pub mod utils;

use faer::prelude::*;
const BLOCK_SIZE: usize = 4;

#[derive(Clone, Debug)]
pub struct YCrBrAMat {
    pub y: Mat<f32>,
    pub cb: Mat<f32>,
    pub cr: Mat<f32>,
    pub a: Mat<f32>,
    /// Dimensions (height, width)
    pub dimensions: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct PaddedYCrBrAMat {
    pub y: Mat<f32>,
    pub cb: Mat<f32>,
    pub cr: Mat<f32>,
    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct DwtedYCrBrAMat {
    /// Y channel components (LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cb channel components (LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cr channel components (LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
}
#[derive(Clone, Debug)]
pub struct BlockCutted {
    // Take LL part only
    pub y_ll_blocks: Vec<Block>,
    pub cb_ll_blocks: Vec<Block>,
    pub cr_ll_blocks: Vec<Block>,

    // Preserved for recovery
    /// Y channel components (LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cb channel components (LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cr channel components (LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
    /// Dimensions of array of blocks (height, width)
    pub blocks_dimensions: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct Block {
    /// 4×4 block matrix
    pub mat_data: Mat<f32>,
    // index in some serial, for strategy purpose
    //pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Imbedded {
    // Take LL part only
    pub y_ll_blocks: Vec<Block>,
    pub cb_ll_blocks: Vec<Block>,
    pub cr_ll_blocks: Vec<Block>,

    // Preserved for recovery
    /// Y channel components (LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cb channel components (LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cr channel components (LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
    /// Block dimensions (height, width)
    pub blocks_dimensions: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct AssembledYCrBrAMat {
    /// Y channel components (LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cb channel components (LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cr channel components (LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
}

impl Imbedded {
    pub fn assemble(self) -> AssembledYCrBrAMat {
        let (block_count_height, block_count_width) = self.blocks_dimensions;
        //write back to ll part of y, cb, cr
        let mut y_ll = self.y.0;
        let mut cb_ll = self.cb.0;
        let mut cr_ll = self.cr.0;

        for i in 0..block_count_height {
            for j in 0..block_count_width {
                y_ll.submatrix_mut(i * BLOCK_SIZE, j * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)
                    .copy_from(&self.y_ll_blocks[i * block_count_width + j].mat_data);
                cb_ll
                    .submatrix_mut(i * BLOCK_SIZE, j * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)
                    .copy_from(&self.cb_ll_blocks[i * block_count_width + j].mat_data);
                cr_ll
                    .submatrix_mut(i * BLOCK_SIZE, j * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)
                    .copy_from(&self.cr_ll_blocks[i * block_count_width + j].mat_data);
            }
        }
        let (_, y_hl, y_lh, y_hh) = self.y;
        let (_, cb_hl, cb_lh, cb_hh) = self.cb;
        let (_, cr_hl, cr_lh, cr_hh) = self.cr;

        AssembledYCrBrAMat {
            y: (y_ll, y_hl, y_lh, y_hh),
            cb: (cb_ll, cb_hl, cb_lh, cb_hh),
            cr: (cr_ll, cr_hl, cr_lh, cr_hh),
            a: self.a,
            original_dimensions: self.original_dimensions,
        }
    }
}
