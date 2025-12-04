use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct WatermarkConfig {
    /// Embedding strength (corresponds to first singular value)
    #[builder(default = "36")]
    pub strength_1: i32,
    /// Embedding strength (corresponds to second singular value), reccommend value is 20
    #[builder(default = "None")]
    pub strength_2: Option<i32>,
    /// Watermark mode
    #[builder(default = "WatermarkMode::Normal")]
    pub mode: WatermarkMode,
}

#[derive(Debug, Clone, Copy)]
pub enum WatermarkMode {
    Normal,
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
