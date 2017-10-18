use luos::module;

pub struct Eye {}

impl Eye {
    pub fn new() -> Eye {
        Eye {}
    }

    pub fn detect(&self) -> bool {
        true
    }
}

impl module::Module for Eye {
    fn alias(&self) -> &'static str {
        "wheel"
    }
}
