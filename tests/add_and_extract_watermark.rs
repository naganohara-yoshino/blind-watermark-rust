use bitvec::prelude::*;
use blind_watermark::prelude::*;
use image::{DynamicImage, ImageReader, Rgba32FImage};

#[test]
fn test_add_and_extract_watermark() {
    let example = ImageReader::open("tests/example.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba32f();
    let ycbcr: YCrBrAMat = example.into();

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
    let processed = DynamicImage::from(processed).to_rgb8();
    processed.save("tests/processed.png").unwrap();

    let processed = ImageReader::open("tests/processed.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba32f();
    let p_ycbcr: YCrBrAMat = processed.into();
    let extracted = p_ycbcr
        .add_padding()
        .dwt()
        .cut()
        .extract_watermark_bits(4, &config);

    assert_eq!(extracted, bitvec![0, 1, 0, 1]);
}
