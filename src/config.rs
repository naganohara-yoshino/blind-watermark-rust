use derive_builder::Builder;

/// Configuration for the watermarking process.
///
/// This struct allows customizing the strength of the watermark embedding and the strategy used.
#[derive(Debug, Clone, Builder)]
pub struct WatermarkConfig {
    /// Embedding strength (corresponds to first singular value).
    ///
    /// Higher values make the watermark more robust but may be more visible.
    /// Default is 36.
    #[builder(default = "36")]
    pub strength_1: i32,
    /// Embedding strength (corresponds to second singular value).
    ///
    /// Recommended value is 20. If `None`, it is not used.
    #[builder(default = "None")]
    pub strength_2: Option<i32>,
    /// Watermark mode.
    ///
    /// Determines how the watermark bits are distributed across the image blocks.
    #[builder(default = "WatermarkMode::Normal")]
    pub mode: WatermarkMode,
}

/// Defines the strategy for distributing watermark bits.
#[derive(Debug, Clone, Copy)]
pub enum WatermarkMode {
    /// Normal mode: Watermark bits are embedded sequentially in the blocks.
    ///
    /// `block_index % watermark_length` determines the bit index.
    Normal,
    /// Strategy mode: Watermark bits are embedded using a random permutation.
    ///
    /// The `u64` value is the seed for the random number generator.
    /// This enhances security by scrambling the watermark location.
    Strategy(u64),
}

impl Default for WatermarkConfig {
    fn default() -> Self {
        Self {
            strength_1: 36,
            strength_2: None,
            mode: WatermarkMode::Normal,
        }
    }
}
