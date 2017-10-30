use luos;
use hal::gpio;

/// Wheel for Electric Dog
pub struct Wheel {
    name: &'static str,
}

impl Wheel {
    /// Setup a Wheel and attach a Servo to the `Pin`
    pub fn new(name: &'static str, _pin: gpio::Pin) -> Wheel {
        Wheel { name }
    }

    /// Make the wheel turn (using the pre-defined speed)
    pub fn on(&self) {}

    /// Stop the wheel
    pub fn off(&self) {}
}

impl luos::Module for Wheel {
    fn alias(&self) -> &'static str {
        self.name
    }
}
