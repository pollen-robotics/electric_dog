use luos;
use hal::adc;

/// Light Sensor for Electric Dog
///
/// Electric Dog has two light sensors in the front left and front right.
///
/// Those modules are responsible for detecting light (true/false). This will be used to make the robot move accordingly.
pub struct LightSensor {
    name: &'static str,
    pin: adc::Input,
    threshold: u8,
}

impl LightSensor {
    /// Initialize the LightSensor and attach it to a `Pin`.
    pub fn new(name: &'static str, pin: adc::Pin) -> LightSensor {
        LightSensor {
            name,
            pin: adc::Input::setup(pin),
            threshold: 127,
        }
    }

    /// Detect if the light is above a predefined threshold.
    pub fn detect(&self) -> bool {
        self.pin.read::<u8>() > self.threshold
    }
}

impl luos::Module for LightSensor {
    fn alias(&self) -> &'static str {
        self.name
    }
}

/// Light detection possibility for Electric Dog
#[derive(Debug)]
pub enum Lights {
    None,
    Left,
    Right,
    Both,
}
