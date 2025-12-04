use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct WatermarkConfig {
    /// Embedding strength (corresponds to d1)
    #[builder(default = "36")]
    pub strength_1: i32,
    /// Embedding strength (corresponds to d2)
    #[builder(default = "20")]
    pub strength_2: i32,
    /// Password
    #[builder(default = "WatermarkMode::Normal")]
    pub mode: WatermarkMode,
}

#[derive(Debug, Clone)]
pub enum WatermarkMode {
    Normal,
    WithPassword(String),
}

impl Default for WatermarkConfig {
    fn default() -> Self {
        Self {
            strength_1: 36,
            strength_2: 20,
            mode: WatermarkMode::Normal,
        }
    }
}
