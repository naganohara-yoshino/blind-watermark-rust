use blind_watermark::prelude::*;

#[test]
fn test_add_and_extract_string_wm() {
    let example = "tests/example.jpg";
    let processed = "tests/processed_string_wm.png";
    let watermark = "こんにちは❗😊";
    let seed = Some(0);
    embed_watermark_string(example, processed, watermark, seed).unwrap();
    let extracted =
        extract_watermark_string(processed, get_wm_len(watermark.as_bytes()), seed).unwrap();
    assert_eq!(extracted, watermark);
}
