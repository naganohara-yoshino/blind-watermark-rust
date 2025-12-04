use crate::{
    config::WatermarkConfig,
    transform::dct::{dct2_2d, dct3_2d},
    Block, BlockCutted, Imbedded,
};
use bitvec::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;

impl BlockCutted {
    /// Embed watermark bits into blocks (Y, Cb, Cr)
    pub fn embed_watermark_bits(
        self,
        watermark_bits: BitVec,
        config: &WatermarkConfig,
    ) -> Imbedded {
        let wm_len = watermark_bits.len();
        let nblocks = self.blocks_dimensions.0 * self.blocks_dimensions.1;

        assert!(nblocks >= wm_len, "not enough blocks for watermark");

        let (y_ll_blocks, cb_ll_blocks, cr_ll_blocks) = (0..nblocks)
            .into_par_iter()
            .map(|i| {
                let bit = watermark_bits[i % wm_len];
                (
                    self.y_ll_blocks[i].imbed_bit(bit, config),
                    self.cb_ll_blocks[i].imbed_bit(bit, config),
                    self.cr_ll_blocks[i].imbed_bit(bit, config),
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .multiunzip();

        Imbedded {
            y_ll_blocks,
            cb_ll_blocks,
            cr_ll_blocks,
            y: self.y,
            cb: self.cb,
            cr: self.cr,
            a: self.a,
            original_dimensions: self.original_dimensions,
            blocks_dimensions: self.blocks_dimensions,
        }
    }

    /// Extract watermark bits with 3-channel majority voting
    pub fn extract_watermark_bits(self, wm_len: usize, config: &WatermarkConfig) -> BitVec {
        let nblocks = self.blocks_dimensions.0 * self.blocks_dimensions.1;

        assert!(wm_len > 0, "wm_len cannot be zero");
        assert!(nblocks > 0);

        // Each block gives one bit from each channel
        let mut y_bits = Vec::with_capacity(nblocks);
        let mut cb_bits = Vec::with_capacity(nblocks);
        let mut cr_bits = Vec::with_capacity(nblocks);

        for i in 0..nblocks {
            y_bits.push(self.y_ll_blocks[i].extract_bit(config));
            cb_bits.push(self.cb_ll_blocks[i].extract_bit(config));
            cr_bits.push(self.cr_ll_blocks[i].extract_bit(config));
        }

        let mut final_bits = BitVec::with_capacity(wm_len);
        for i in 0..wm_len {
            // gather bits from all blocks that correspond to this watermark index
            let mut sum = 0u32;
            let mut count = 0u32;

            let mut j = i;
            while j < nblocks {
                // majority over Y/Cb/Cr for this block
                let block_sum = y_bits[j] as u8 + cb_bits[j] as u8 + cr_bits[j] as u8;
                sum += block_sum as u32;
                count += 3;
                j += wm_len;
            }

            // final recovered bit
            final_bits.push(sum * 2 >= count); // average > 0.5
        }

        final_bits
    }
}

impl Block {
    fn imbed_bit(&self, bit: bool, config: &WatermarkConfig) -> Block {
        // Attempt SVD on the current matrix; fallback to original block if it fails
        let Ok(svd_output) = dct2_2d(self.mat_data.as_ref()).svd() else {
            return self.clone();
        };

        // Retrieve the left and right singular matrices
        let u = svd_output.U();
        let v = svd_output.V();

        // Hack: convert a read-only MatRef to an owned, mutable Mat
        let mut s = svd_output.S() * 1.0;

        // Retrieve quantization strength
        let strength = config.strength_1;

        // Modify the primary singular value to embed the bit
        s[0] = embed_quantization(s[0], bit, strength);

        // Reconstruct the matrix and return a new Block

        let mat_data = dct3_2d((u * s * v.transpose()).as_ref());
        Block { mat_data }
    }

    fn extract_bit(&self, config: &WatermarkConfig) -> bool {
        // Retrieve singular values; return false if unavailable
        let Ok(singular) = dct2_2d(self.mat_data.as_ref()).singular_values() else {
            return false;
        };

        // Extract the bit from the primary singular value
        extract_quantization(singular[0], config.strength_1)
    }
}

/// Quantizes a singular value based on the bit and strength
fn embed_quantization(target: f32, bit: bool, strength: i32) -> f32 {
    let target = target * 255.0;
    let f_strength = strength as f32;
    (((target / f_strength).floor() + if bit { 3.0 / 4.0 } else { 1.0 / 4.0 }) * f_strength) / 255.0
}

/// Extracts the bit from a singular value using the quantization strength
fn extract_quantization(target: f32, strength: i32) -> bool {
    let target = target * 255.0;
    let f_strength = strength as f32;
    target % f_strength > f_strength / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use faer::prelude::*; // Assuming faer_core for matrix creation

    /// Helper to create a simple test Block
    fn create_test_block() -> Block {
        let data = Mat::<f32>::full(4, 4, 1.0);
        Block {
            mat_data: data.into(),
        }
    }

    #[test]
    fn test_embed_extract_bit_true() {
        let block = create_test_block();
        let config = WatermarkConfig::default();

        // Embed a true bit
        let watermarked = block.imbed_bit(true, &config);

        // Extract the bit
        let extracted = watermarked.extract_bit(&config);

        assert!(extracted, "Embedded true bit should be extracted as true");
    }

    #[test]
    fn test_embed_extract_bit_false() {
        let block = create_test_block();
        let config = WatermarkConfig::default();

        // Embed a false bit
        let watermarked = block.imbed_bit(false, &config);

        // Extract the bit
        let extracted = watermarked.extract_bit(&config);

        assert!(
            !extracted,
            "Embedded false bit should be extracted as false"
        );
    }
}
