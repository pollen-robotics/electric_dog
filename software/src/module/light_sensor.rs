pub struct LightSensor {}

impl LightSensor {
    pub fn detect(&self) -> bool {
        true
    }
}

/// Light
pub enum Lights {
    None,
    Left,
    Right,
    Both,
}
