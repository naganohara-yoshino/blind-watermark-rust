use blind_watermark::prelude::*;

fn main() {
    let processed = "processed.png";
    let watermark_len = get_wm_len("こんにちは❗😊".as_bytes());
    let seed = Some(0);
    let extracted = extract_watermark_string(processed, watermark_len, seed).unwrap();
    println!("Extracted bits: {:?}", extracted);
}
