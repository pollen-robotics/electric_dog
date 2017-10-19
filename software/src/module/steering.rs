use luos;
use hal::{gpio, servo};

/// Steering Wheel for Electric Dog
///
/// This is used to steer Electric Dog. The wheel can only take three pre-defined position to:
///
/// * turn left
/// * move forward
/// * turn right
///
pub struct SteeringWheel {
    name: &'static str,
    servo: servo::Servo,
}

impl SteeringWheel {
    /// Setup and attach to a `Pin` used for the Servo.
    pub fn new(name: &'static str, pin: gpio::Pin) -> SteeringWheel {
        SteeringWheel {
            name,
            servo: servo::attach(pin),
        }
    }
    /// Set the Steering Wheel in one of the pre-defined position.
    pub fn set(&self, steering: &Steering) {
        let position = match *steering {
            Steering::Center => 0.0,
            Steering::Left => -90.0,
            Steering::Right => 90.0,
        };

        self.servo.write(position);
    }
}

impl luos::Module for SteeringWheel {
    fn alias(&self) -> &'static str {
        self.name
    }
}

/// Steering position possibilities
pub enum Steering {
    Center,
    Left,
    Right,
}
