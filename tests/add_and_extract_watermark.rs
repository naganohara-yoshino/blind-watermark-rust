use bitvec::prelude::*;
use blind_watermark::prelude::*;

#[test]
fn test_add_and_extract_watermark() {
    let example = "tests/example.jpg";
    let processed = "tests/processed.png";
    let watermark = bitvec![0, 1, 0, 1];
    let seed = 0;
    embed_watermark(example, processed, watermark.clone(), seed).unwrap();
    let extracted = extract_watermark(processed, 4, seed).unwrap();
    assert_eq!(extracted, watermark);
}
