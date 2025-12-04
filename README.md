# Blind Watermark

A Rust library for blind image watermarking using DWT (Discrete Wavelet Transform), DCT (Discrete Cosine Transform), and SVD (Singular Value Decomposition). The algorithm is originally implemented in python by *Guo Fei* at [this repository](https://github.com/guofei9987/blind_watermark/). This is the rust rewritten version.

## Features

- **Blind Watermarking**: Extract the watermark without needing the original image.
- **Robust Algorithm**: Combines DWT, DCT, and SVD for embedding watermarks in the frequency domain.
- **High Performance**: Developed in Rust, leveraging the `faer` crate for efficient matrix computations and `rayon` for multi-threading support.
- **Flexible**: Supports embedding arbitrary binary data.
- **Random Strategy**: Supports randomized block selection for embedding watermarks, enhancing security.
- **High-Level API**: Provides a fluent builder-like API for easy integration.

### Original Image
![Original](/tests/example.jpg)

### Processed Image
![Processed](/tests/processed.png)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
blind_watermark = "0.1.0"
```

## Usage

### Embedding a Watermark

```rust
use bitvec::prelude::*;
use blind_watermark::prelude::*;
use image::{DynamicImage, ImageReader, Rgba32FImage};

fn main() {
    // 1. Load the image using the `image` crate
    let img = ImageReader::open("input.jpg")
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .into_rgba32f();

    // 2. Convert to YCbCrA format
    let ycbcr: YCrBrAMat = img.into();

    // 3. Define watermark (bits)
    let watermark = bitvec![0, 1, 0, 1, 1, 0, 1, 0]; // Example bits
    
    // 4. Configure watermark settings (Optional)
    let config = WatermarkConfigBuilder::default()
        .strength_1(36)
        .mode(WatermarkMode::Strategy(12345)) // Use random strategy with seed
        .build()
        .unwrap();

    // 5. Process pipeline: Padding -> DWT -> Cut Blocks -> Embed -> Assemble -> IDWT -> Remove Padding
    let processed = ycbcr
        .add_padding()
        .dwt()
        .cut()
        .embed_watermark_bits(watermark, &config)
        .assemble()
        .idwt()
        .remove_padding();

    // 6. Save the result
    let processed_image: Rgba32FImage = processed.into();
    let output_image: DynamicImage = processed_image.into();
    output_image.to_rgb8().save("watermarked.png").unwrap();
}
```

### Extracting a Watermark

To extract the watermark, you only need the watermarked image and the length of the watermark.

```rust
use bitvec::prelude::*;
use blind_watermark::prelude::*;
use image::ImageReader;

fn main() {
    // 1. Load the watermarked image
    let img = ImageReader::open("watermarked.png")
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .into_rgba32f();

    let ycbcr: YCrBrAMat = img.into();
    
    // 2. Use the same config as embedding
    let config = WatermarkConfigBuilder::default()
        .strength_1(36)
        .mode(WatermarkMode::Strategy(12345))
        .build()
        .unwrap();
        
    let watermark_len = 8; // Length of the embedded watermark

    // 3. Process pipeline: Padding -> DWT -> Cut Blocks -> Extract
    let extracted_bits = ycbcr
        .add_padding()
        .dwt()
        .cut()
        .extract_watermark_bits(watermark_len, &config);

    println!("Extracted bits: {:?}", extracted_bits);
}
```

## Algorithm Details

The library implements a hybrid DWT-DCT-SVD watermarking scheme:

1.  **Preprocessing**: The image is converted to YCbCr color space.
2.  **DWT**: A Discrete Wavelet Transform is applied to decompose the image into frequency subbands (LL, HL, LH, HH).
3.  **Block Selection**: The LL (Low-Low) subband is divided into 4x4 blocks.
4.  **DCT & SVD**: Each block undergoes Discrete Cosine Transform (DCT) followed by Singular Value Decomposition (SVD).
5.  **Embedding**: The watermark bits are embedded by quantizing the singular values of the blocks.
6.  **Reconstruction**: The inverse transformations (ISVD, IDCT, IDWT) are applied to generate the watermarked image.

This approach ensures that the watermark is embedded in the significant features of the image, providing robustness while maintaining visual quality.

## License

Licensed under either of

*   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
*   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
