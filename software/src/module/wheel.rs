use hal::{gpio, servo};

pub struct Wheel {
    servo: servo::Servo,
    speed: f32,
}

impl Wheel {
    pub fn new(pin: gpio::Pin) -> Wheel {
        Wheel {
            servo: servo::attach(pin),
            speed: 50.0,
        }
    }

    pub fn on(&self) {
        self.servo.write(self.speed);
    }

    pub fn off(&self) {
        self.servo.write(0.0);
    }
}
