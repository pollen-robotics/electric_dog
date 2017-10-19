use hal::{gpio, servo};

/// Steering Wheel
pub struct SteeringWheel {
    servo: servo::Servo,
}

impl SteeringWheel {
    pub fn new(pin: gpio::Pin) -> SteeringWheel {
        SteeringWheel { servo: servo::attach(pin) }
    }
    pub fn set(&self, steering: &Steering) {
        let position = match *steering {
            Steering::Center => 0.0,
            Steering::Left => -90.0,
            Steering::Right => 90.0,
        };

        self.servo.write(position);
    }
}

pub enum Steering {
    Center,
    Left,
    Right,
}
