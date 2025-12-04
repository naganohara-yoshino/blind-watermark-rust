use anyhow::Result;
use bitvec::prelude::*;
use image::{DynamicImage, ImageReader, Rgba32FImage};

use crate::{
    config::{WatermarkConfigBuilder, WatermarkMode},
    YCrBrAMat,
};

/// Extracts a watermark from an image using the specified strategy.
///
/// # Arguments
///
/// * `img_path` - Path to the watermarked image.
/// * `wm_len` - Length of the watermark in bits.
/// * `seed` - Seed used for the random strategy during embedding.
///
/// # Returns
///
/// The extracted watermark as a `BitVec`.
pub fn extract_watermark(img_path: &str, wm_len: usize, seed: u64) -> Result<BitVec> {
    let img = ImageReader::open(img_path)?.decode()?.into_rgba32f();

    let ycbcr: YCrBrAMat = img.into();
    let config = WatermarkConfigBuilder::default()
        .mode(WatermarkMode::Strategy(seed))
        .build()?;
    Ok(ycbcr
        .add_padding()
        .dwt()
        .cut()
        .extract_watermark_bits(wm_len, &config))
}

/// Embeds a watermark into an image using the specified strategy.
///
/// # Arguments
///
/// * `img_path` - Path to the input image.
/// * `img_output_path` - Path to save the watermarked image.
/// * `watermark` - The watermark bits to embed.
/// * `seed` - Seed for the random strategy.
pub fn embed_watermark(
    img_path: &str,
    img_output_path: &str,
    watermark: BitVec,
    seed: u64,
) -> Result<()> {
    let img = ImageReader::open(img_path)?.decode()?.into_rgba32f();
    let ycbcr: YCrBrAMat = img.into();
    let config = WatermarkConfigBuilder::default()
        .mode(WatermarkMode::Strategy(seed))
        .build()?;
    let processed = ycbcr
        .add_padding()
        .dwt()
        .cut()
        .embed_watermark_bits(watermark, &config)
        .assemble()
        .idwt()
        .remove_padding();
    let processed_image: Rgba32FImage = processed.into();
    let output_image: DynamicImage = processed_image.into();
    output_image.to_rgb8().save(img_output_path)?;
    Ok(())
}
