pub struct Light {}

impl Light {
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
