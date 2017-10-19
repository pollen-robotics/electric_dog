use luos;
use hal::{gpio, servo};

/// Wheel for Electric Dog
pub struct Wheel {
    name: &'static str,
    servo: servo::Servo,
    speed: f32,
}

impl Wheel {
    /// Setup a Wheel and attach a Servo to the `Pin`
    pub fn new(name: &'static str, pin: gpio::Pin) -> Wheel {
        Wheel {
            name,
            servo: servo::attach(pin),
            speed: 50.0,
        }
    }

    /// Make the wheel turn (using the pre-defined speed)
    pub fn on(&self) {
        self.servo.write(self.speed);
    }

    /// Stop the wheel
    pub fn off(&self) {
        self.servo.write(0.0);
    }
}

impl luos::Module for Wheel {
    fn alias(&self) -> &'static str {
        self.name
    }
}
