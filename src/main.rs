use std::f32::consts::PI;

use blind_watermark::{Block, transform::dct};
use faer::prelude::*;
const SQUARE_BLOCK_SIZE: usize = 4;
use blind_watermark::transform::dct::{dct2_2d, dct3_2d};
fn main() {
    let a = mat![[1.0, 2.0], [3.0, 4.0],];
    let b = dct2_2d(a.as_ref());
    let c = dct3_2d(b.as_ref());
    eprintln!("a =\n {:?}\n", a);
    eprintln!("b =\n {:?}\n", b);
    eprintln!("c =\n {:?}\n", c);
}
