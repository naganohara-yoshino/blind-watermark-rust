use anyhow::Result;
use bitvec::prelude::*;
use image::{DynamicImage, ImageReader, Rgba32FImage};

use crate::{
    config::{WatermarkConfig, WatermarkConfigBuilder, WatermarkMode},
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
pub fn extract_watermark_bits(
    img_path: &str,
    wm_len: usize,
    seed: Option<u64>,
) -> Result<BitVec<u8>> {
    let img = ImageReader::open(img_path)?.decode()?.into_rgba32f();

    let ycbcr: YCrBrAMat = img.into();

    let config = match seed {
        None => WatermarkConfig::default(),
        Some(seed) => WatermarkConfigBuilder::default()
            .mode(WatermarkMode::Strategy(seed))
            .strength_2(20)
            .build()?,
    };
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
pub fn embed_watermark_bits(
    img_path: &str,
    img_output_path: &str,
    watermark: &BitSlice<u8>,
    seed: Option<u64>,
) -> Result<()> {
    let img = ImageReader::open(img_path)?.decode()?.into_rgba32f();
    let ycbcr: YCrBrAMat = img.into();
    let config = match seed {
        None => WatermarkConfig::default(),
        Some(seed) => WatermarkConfigBuilder::default()
            .mode(WatermarkMode::Strategy(seed))
            .strength_2(20)
            .build()?,
    };
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

/// Embeds a watermark into an image using the specified strategy.
///
/// # Arguments
///
/// * `img_path` - Path to the input image.
/// * `img_output_path` - Path to save the watermarked image.
/// * `watermark` - The watermark bits to embed.
/// * `seed` - Seed for the random strategy.
pub fn embed_watermark_bytes(
    img_path: &str,
    img_output_path: &str,
    watermark: &[u8],
    seed: Option<u64>,
) -> Result<()> {
    let bv = watermark.as_bits::<Lsb0>();
    embed_watermark_bits(img_path, img_output_path, bv, seed)?;
    Ok(())
}

pub fn get_wm_len(watermark: &[u8]) -> usize {
    watermark.len() * 8
}

/// Extracts a watermark from an image using the specified strategy.
///
/// # Arguments
///
/// * `img_path` - Path to the watermarked image.
/// * `wm_len` - Length of the watermark in bits.
/// * `seed` - Seed used for the random strategy during embedding.
pub fn extract_watermark_bytes(
    img_path: &str,
    wm_len: usize,
    seed: Option<u64>,
) -> Result<Vec<u8>> {
    let bv = extract_watermark_bits(img_path, wm_len, seed)?;
    Ok(bv.into_vec())
}

/// Extracts a watermark from an image using the specified strategy.
///
/// # Arguments
///
/// * `img_path` - Path to the watermarked image.
/// * `wm_len` - Length of the watermark in bits.
/// * `seed` - Seed used for the random strategy during embedding.
pub fn extract_watermark_string(
    img_path: &str,
    wm_len: usize,
    seed: Option<u64>,
) -> Result<String> {
    let bytes = extract_watermark_bytes(img_path, wm_len, seed)?;
    let s = String::from_utf8(bytes)?;
    Ok(s)
}

/// Embeds a watermark into an image using the specified strategy.
///
/// # Arguments
///
/// * `img_path` - Path to the input image.
/// * `img_output_path` - Path to save the watermarked image.
/// * `watermark` - The watermark bits to embed.
/// * `seed` - Seed for the random strategy.
pub fn embed_watermark_string(
    img_path: &str,
    img_output_path: &str,
    watermark: &str,
    seed: Option<u64>,
) -> Result<()> {
    let bytes = watermark.as_bytes();
    embed_watermark_bytes(img_path, img_output_path, bytes, seed)
}
