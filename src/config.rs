#[derive(Debug, Clone)]
pub struct WatermarkConfig {
    /// 嵌入强度 (对应 d1)
    pub strength_low: f32,
    /// 随机种子/密码
    pub password: Option<u64>,
}

impl Default for WatermarkConfig {
    fn default() -> Self {
        Self {
            strength_low: 36.0,
            password: None,
        }
    }
}

// pub struct ConfigBuilder {
//     config: WatermarkConfig,
// }

// impl ConfigBuilder {
//     pub fn new() -> Self { /* ... */
//     }
//     pub fn password(mut self, pwd: u64) -> Self { /* ... */
//     }
//     pub fn strength(mut self, d1: f32) -> Self { /* ... */
//     }
//     pub fn build(self) -> WatermarkConfig {
//         self.config
//     }
// }
