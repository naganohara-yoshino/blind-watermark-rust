use bitvec::prelude::*;
use blind_watermark::prelude::*;
use image::{DynamicImage, ImageReader, RgbImage, Rgba32FImage};
fn main() {
    let img = ImageReader::open("example.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba32f();
    let ycbcr: YCrBrAMat = img.into();

    let config = WatermarkConfig::default();
    let processed = ycbcr
        .add_padding()
        .dwt()
        .cut()
        .embed_watermark_bits(bitvec![0, 1, 0, 1], &config)
        .assemble()
        .idwt()
        .remove_padding();

    let processed: Rgba32FImage = processed.into();
    let o_prime: DynamicImage = processed.into();
    let o = o_prime.to_rgb8();
    o.save("processed.png").unwrap();
}
