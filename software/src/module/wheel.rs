use luos::module;

pub struct Wheel {}

impl Wheel {
    pub fn new() -> Wheel {
        Wheel {}
    }
    pub fn on(&self) {}
    pub fn off(&self) {}
}

impl module::Module for Wheel {
    fn alias(&self) -> &'static str {
        "wheel"
    }
}
