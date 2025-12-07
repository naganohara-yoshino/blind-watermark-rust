use crate::{
    Block, BlockCutted, Imbedded,
    config::{WatermarkConfig, WatermarkMode},
    quantization::{average_value, embed_quantization, extract_quantization},
    strategy::Permutation,
    transform::dct::{dct2_2d, dct3_2d},
};
use bitvec::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;

impl BlockCutted {
    /// Embed watermark bits into blocks (Y, Cb, Cr)
    pub fn embed_watermark_bits(
        self,
        watermark_bits: &BitSlice<u8>,
        config: &WatermarkConfig,
    ) -> Imbedded {
        let wm_len = watermark_bits.len();
        let nblocks = self.blocks_dimensions.0 * self.blocks_dimensions.1;

        assert!(nblocks >= wm_len, "not enough blocks for watermark");

        let perm = match config.mode {
            WatermarkMode::Normal => Permutation {
                f: (0..nblocks).collect(),
                n: nblocks,
            },
            WatermarkMode::Strategy(seed) => Permutation::new(nblocks, seed),
        };

        let (y_ll_blocks, cb_ll_blocks, cr_ll_blocks) = (0..nblocks)
            .into_par_iter()
            .map(|i| {
                let bit = watermark_bits[perm.corresponding_wmbits_position(i, wm_len)];
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

    /// Extract watermark bits using 3-channel majority voting (parallelized and optimized)
    pub fn extract_watermark_bits(self, wm_len: usize, config: &WatermarkConfig) -> BitVec<u8> {
        let nblocks = self.blocks_dimensions.0 * self.blocks_dimensions.1;

        assert!(wm_len > 0, "wm_len cannot be zero");
        assert!(nblocks > 0);

        let perm = match config.mode {
            WatermarkMode::Normal => Permutation {
                f: (0..nblocks).collect(),
                n: nblocks,
            },
            WatermarkMode::Strategy(seed) => Permutation::new(nblocks, seed),
        };

        // 1. Parallel extraction of bits for Y, Cb and Cr at each block position `i`.
        let block_bits: Vec<_> = (0..nblocks)
            .into_par_iter()
            .map(|i| {
                (
                    self.y_ll_blocks[i].extract_bit(config),
                    self.cb_ll_blocks[i].extract_bit(config),
                    self.cr_ll_blocks[i].extract_bit(config),
                )
            })
            .collect();

        // 2. Parallel majority voting for deciding each watermark bit at bitvec position `i`.
        (0..wm_len)
            // .into_par_iter()
            .map(|i| {
                let corresponding_block_positions = perm.corresponding_block_positions(i, wm_len);

                // Sum over the possible blocks corresponding to this watermark bit `i`
                let total = corresponding_block_positions
                    .iter()
                    .map(|&j| {
                        block_bits[j].0 as usize
                            + block_bits[j].1 as usize
                            + block_bits[j].2 as usize
                    })
                    .sum::<usize>();
                let count = corresponding_block_positions.len() * 3;

                // Majority voting: return true if most of the corresponding value is `true`, vice versa.
                total * 2 >= count
            })
            .collect::<Vec<bool>>()
            .into_iter()
            .collect() // Convert to `BitVec
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
        let strength_1 = config.strength_1;

        // Modify the primary singular value to embed the bit
        s[0] = embed_quantization(s[0], bit, strength_1);

        if let Some(strength_2) = config.strength_2 {
            s[1] = embed_quantization(s[1], bit, strength_2);
        }

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
        let strength_1 = config.strength_1;

        match config.strength_2 {
            None => extract_quantization(singular[0], strength_1),
            Some(strength_2) => {
                let first = extract_quantization(singular[0], strength_1);
                let second = extract_quantization(singular[1], strength_2);
                average_value(first, second)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{WatermarkConfig, WatermarkConfigBuilder, WatermarkMode};
    use faer::prelude::*;

    /// Helper to create a simple test Block
    fn create_test_block() -> Block {
        let data = Mat::<f32>::full(4, 4, 1.0);
        Block { mat_data: data }
    }

    fn create_test_config() -> WatermarkConfig {
        WatermarkConfigBuilder::default()
            .mode(WatermarkMode::Strategy(0))
            .build()
            .unwrap()
    }

    #[test]
    fn test_embed_extract_bit_true() {
        let block = create_test_block();
        let config = create_test_config();

        // Embed a true bit
        let watermarked = block.imbed_bit(true, &config);

        // Extract the bit
        let extracted = watermarked.extract_bit(&config);

        assert!(extracted, "Embedded true bit should be extracted as true");
    }

    #[test]
    fn test_embed_extract_bit_false() {
        let block = create_test_block();
        let config = create_test_config();

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
