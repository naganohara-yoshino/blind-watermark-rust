use anyhow::Result;
use bitvec::prelude::*;
use image::{DynamicImage, ImageReader, Rgba32FImage};

use crate::{
    config::{WatermarkConfigBuilder, WatermarkMode},
    YCrBrAMat,
};

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
    output_image.to_rgb8().save(img_output_path).unwrap();
    Ok(())
}
