use bitvec::prelude::*;
use blind_watermark::prelude::*;

fn main() {
    let example = "example.jpg";
    let processed = "processed.png";
    let watermark = bitvec![0, 1, 0, 1];
    let seed = 0;
    embed_watermark(example, processed, watermark.clone(), seed).unwrap();
}
