use crate::{Block, config::WatermarkConfig};

impl Block {
    fn imbed_bit(&self, bit: bool, config: &WatermarkConfig) -> Block {
        // Attempt SVD on the current matrix; fallback to original block if it fails
        let Ok(svd_output) = self.mat_data.as_ref().svd() else {
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
        Block {
            mat_data: u * s * v.transpose(),
        }
    }

    fn extract_bit(&self, config: &WatermarkConfig) -> bool {
        // Retrieve singular values; return false if unavailable
        let Ok(singular) = self.mat_data.as_ref().singular_values() else {
            return false;
        };

        // Extract the bit from the primary singular value
        extract_quantization(singular[0], config.strength_1)
    }
}

/// Quantizes a singular value based on the bit and strength
fn embed_quantization(target: f32, bit: bool, strength: i32) -> f32 {
    let f_strength = strength as f32;
    ((target / f_strength).floor() + if bit { 3.0 / 4.0 } else { 1.0 / 4.0 }) * f_strength
}

/// Extracts the bit from a singular value using the quantization strength
fn extract_quantization(target: f32, strength: i32) -> bool {
    let f_strength = strength as f32;
    target % f_strength > f_strength / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use faer::prelude::*; // Assuming faer_core for matrix creation

    /// Helper to create a simple test Block
    fn create_test_block() -> Block {
        let data= Mat::<f32>::full(4,4 , 1.0);
        Block { mat_data: data.into() }
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
