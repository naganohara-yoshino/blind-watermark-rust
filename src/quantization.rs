/// Quantizes a singular value based on the bit and strength
pub fn embed_quantization(target: f32, bit: bool, strength: i32) -> f32 {
    let target = target * 255.0;
    let f_strength = strength as f32;
    (((target / f_strength).floor() + if bit { 3.0 / 4.0 } else { 1.0 / 4.0 }) * f_strength) / 255.0
}

/// Extracts the bit from a singular value using the quantization strength
pub fn extract_quantization(target: f32, strength: i32) -> bool {
    let target = target * 255.0;
    let f_strength = strength as f32;
    target % f_strength > f_strength / 2.0
}
