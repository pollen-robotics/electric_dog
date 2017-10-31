use luos;
use hal::gpio;

/// Wheel for Electric Dog
pub struct Wheel {
    name: &'static str,
    pin: gpio::Output,
}

impl Wheel {
    /// Setup a Wheel and attach a Servo to the `Pin`
    pub fn new(name: &'static str, pin: gpio::Pin) -> Wheel {
        Wheel {
            name,
            pin: gpio::Output::setup(pin),
         }
    }

    /// Make the wheel turn (using the pre-defined speed)
    pub fn on(&mut self) {
        self.pin.high();
    }

    /// Stop the wheel
    pub fn off(&mut self) {
        self.pin.low();
    }
}

impl luos::Module for Wheel {
    fn alias(&self) -> &'static str {
        self.name
    }
}
