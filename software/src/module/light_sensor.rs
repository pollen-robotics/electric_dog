use luos;
use hal::gpio;

/// Light Sensor for Electric Dog
///
/// Electric Dog has two light sensors in the front left and front right.
///
/// Those modules are responsible for detecting light (true/false). This will be used to make the robot move accordingly.
pub struct LightSensor {
    name: &'static str,
    pin: gpio::Pin,
    threshold: u16,
}

impl LightSensor {
    /// Initialize the LightSensor and attach it to a `Pin`.
    pub fn new(name: &'static str, pin: gpio::Pin) -> LightSensor {
        LightSensor {
            name,
            pin,
            threshold: 512,
        }
    }

    /// Detect if the light is above a predefined threshold.
    pub fn detect(&self) -> bool {
        gpio::analog_read::<u16>(&self.pin) > self.threshold
    }
}

impl luos::Module for LightSensor {
    fn alias(&self) -> &'static str {
        self.name
    }
}

/// Light detection possibility for Electric Dog
pub enum Lights {
    None,
    Left,
    Right,
    Both,
}
