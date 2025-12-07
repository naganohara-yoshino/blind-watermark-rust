//! # Blind Watermark

//! A Rust library for blind image watermarking using DWT (Discrete Wavelet Transform), DCT (Discrete Cosine Transform), and SVD (Singular Value Decomposition). The algorithm is originally implemented in python by *Guo Fei* at [this repository](https://github.com/guofei9987/blind_watermark/). This is the rust rewritten version.
//!
//! ## Features
//!
//! - **Blind Watermarking**: Extract the watermark without needing the original image.
//! - **Robust Algorithm**: Combines DWT, DCT, and SVD for embedding watermarks in the frequency domain.
//! - **High Performance**: Developed in Rust, leveraging the `faer` crate for efficient matrix computations and `rayon` for multi-threading support.
//! - **Flexible**: Supports embedding arbitrary binary data (e.g. `Vec<u8>`). String watermarking is natively supported.
//！- **Random Strategy**: Supports randomized block selection for embedding watermarks, enhancing security.
//！- **High-Level API**: Provides a fluent API for easy integration.

pub mod config;
pub mod prelude;
pub(crate) mod quantization;
pub mod strategy;
pub mod transform;
pub mod utils;

use faer::prelude::*;
const BLOCK_SIZE: usize = 4;

/// Matrix representation of an image in YCbCrA color space.
///
/// This struct holds the image data separated into Y (Luminance), Cb (Blue-difference),
/// Cr (Red-difference), and A (Alpha) channels.
#[derive(Clone, Debug)]
pub struct YCrBrAMat {
    /// Y channel (Luminance)
    pub y: Mat<f32>,
    /// Cb channel (Blue-difference)
    pub cb: Mat<f32>,
    /// Cr channel (Red-difference)
    pub cr: Mat<f32>,
    /// Alpha channel
    pub a: Mat<f32>,
    /// Dimensions (height, width)
    pub dimensions: (usize, usize),
}

/// Padded matrix representation of an image in YCbCrA color space.
///
/// Padding is added to ensure the image dimensions are suitable for DWT (Discrete Wavelet Transform).
#[derive(Clone, Debug)]
pub struct PaddedYCrBrAMat {
    /// Y channel (Luminance) with padding
    pub y: Mat<f32>,
    /// Cb channel (Blue-difference) with padding
    pub cb: Mat<f32>,
    /// Cr channel (Red-difference) with padding
    pub cr: Mat<f32>,
    /// Alpha channel with padding
    pub a: Mat<f32>,
    /// Original dimensions (height, width) before padding
    pub original_dimensions: (usize, usize),
}

/// Image data after Discrete Wavelet Transform (DWT).
///
/// The DWT decomposes the image into four subbands: LL (Approximation), HL (Horizontal Detail),
/// LH (Vertical Detail), and HH (Diagonal Detail).
#[derive(Clone, Debug)]
pub struct DwtedYCrBrAMat {
    /// Y channel components (LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cb channel components (LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cr channel components (LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    /// Alpha channel (not transformed)
    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
}
/// Image data with the LL subband divided into blocks.
///
/// This struct holds the blocks prepared for watermark embedding/extraction, along with
/// the other subbands preserved for reconstruction.
#[derive(Clone, Debug)]
pub struct BlockCutted {
    // Take LL part only
    /// Blocks from Y channel LL subband
    pub y_ll_blocks: Vec<Block>,
    /// Blocks from Cb channel LL subband
    pub cb_ll_blocks: Vec<Block>,
    /// Blocks from Cr channel LL subband
    pub cr_ll_blocks: Vec<Block>,

    // Preserved for recovery
    /// Y channel components (LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cb channel components (LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cr channel components (LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    /// Alpha channel
    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
    /// Dimensions of array of blocks (height, width)
    pub blocks_dimensions: (usize, usize),
}

/// A single block of the image (typically 4x4).
#[derive(Clone, Debug)]
pub struct Block {
    /// 4×4 block matrix
    pub mat_data: Mat<f32>,
    // index in some serial, for strategy purpose
    //pub index: usize,
}

/// Image data with watermark embedded in the blocks.
#[derive(Clone, Debug)]
pub struct Imbedded {
    // Take LL part only
    /// Watermarked blocks from Y channel LL subband
    pub y_ll_blocks: Vec<Block>,
    /// Watermarked blocks from Cb channel LL subband
    pub cb_ll_blocks: Vec<Block>,
    /// Watermarked blocks from Cr channel LL subband
    pub cr_ll_blocks: Vec<Block>,

    // Preserved for recovery
    /// Y channel components (LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cb channel components (LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cr channel components (LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    /// Alpha channel
    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
    /// Block dimensions (height, width)
    pub blocks_dimensions: (usize, usize),
}

/// Image data assembled back from blocks and subbands.
///
/// This struct represents the state after reassembling the blocks into the LL subband,
/// ready for Inverse DWT.
#[derive(Clone, Debug)]
pub struct AssembledYCrBrAMat {
    /// Y channel components (LL, HL, LH, HH)
    pub y: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cb channel components (LL, HL, LH, HH)
    pub cb: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),
    /// Cr channel components (LL, HL, LH, HH)
    pub cr: (Mat<f32>, Mat<f32>, Mat<f32>, Mat<f32>),

    /// Alpha channel
    pub a: Mat<f32>,
    /// Original dimensions (height, width)
    pub original_dimensions: (usize, usize),
}

impl Imbedded {
    /// Assembles the blocks back into the LL subband of the Y, Cb, and Cr channels.
    ///
    /// This reverses the block cutting process, preparing the data for the Inverse DWT.
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
