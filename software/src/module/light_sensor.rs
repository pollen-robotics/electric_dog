use hal::gpio;

pub struct LightSensor {
    pin: gpio::Pin,
    threshold: u16,
}

impl LightSensor {
    pub fn new(pin: gpio::Pin) -> LightSensor {
        LightSensor {
            pin,
            threshold: 512,
        }
    }

    pub fn detect(&self) -> bool {
        gpio::analog_read::<u16>(&self.pin) > self.threshold
    }
}

pub enum Lights {
    None,
    Left,
    Right,
    Both,
}
