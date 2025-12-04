use bitvec::prelude::*;
use blind_watermark::prelude::*;

#[test]
fn test_add_and_extract_watermark() {
    let example = "tests/example.jpg";
    let processed = "tests/processed_plain.png";
    let watermark = bits![u8, Lsb0; 0, 1, 0, 1];
    let seed = None;
    embed_watermark_bits(example, processed, &watermark, seed).unwrap();
    let extracted = extract_watermark_bits(processed, 4, seed).unwrap();
    assert_eq!(extracted, watermark);
}
