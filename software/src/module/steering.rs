use luos;
use hal::pwm;

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
}

impl SteeringWheel {
    /// Setup and attach to a `Pin` used for the Servo.
    pub fn new(name: &'static str) -> SteeringWheel {
        pwm::init(10000);
        pwm::set_duty(500);
        pwm::enable();

        SteeringWheel { name }
    }
    /// Set the Steering Wheel in one of the pre-defined position.
    pub fn set(&self, steering: &Steering) {
        let duty = match *steering {
            Steering::Center => 500,
            Steering::Left => 250,
            Steering::Right => 750,
        };
        pwm::set_duty(duty);
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
