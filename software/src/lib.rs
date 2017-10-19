//! # Electric Dog
//!
//! This crate provides all the software used in the [Replica of the robot Electric Dog](https://github.com/pollen/electric_dog).
//!
//! It is based upon [luos](https://github.com/pollen/luos). You will find the different assets used for creating the luos modules composing the robot. You will also find the luos app. for creating its *following the light* behavior.
//!
//! ## The Electric Dog Modules
//!
//! * Wheel (left and right)
//! * a Steering Wheel (rear)
//! * Light Sensor (left and right)
//! * a Remote Control Receiver
//!
//! ## Follow the light behavior
//!
//! The robot will start moving as soon as one of its light sensor detects some lights:
//!
//! * If both light sensor detects something, the robot will move forward.
//! * If only one of them detects light, the robot will turn towards the light.
//! * And if the robot doesn't detect light, it will stop.

extern crate luos;

extern crate mockup_hal as hal;
use hal::gpio;

pub mod module;

use module::Wheel;
use module::{Steering, SteeringWheel};
use module::{LightSensor, Lights};
use module::RemoteControlReceiver;

/// ElectricDog struct representing the whole robot.
///
/// ## Modules
///
/// * both wheels (left, right)
/// * the steering wheel
/// * both light sensor (left, right)
/// * the remote control receiver
///
/// ## Examples
///
/// ```
/// use std::{thread, time};
/// use electric_dog::ElectricDog;
///
/// let mut electric_dog = ElectricDog::new();
///
/// for _ in 0..4 {
///     electric_dog.move_forward();
///     thread::sleep(time::Duration::from_secs(1));
///     electric_dog.turn_left();
///     thread::sleep(time::Duration::from_secs(1));
/// }
///
/// electric_dog.stop();
/// ```
///
pub struct ElectricDog {
    pub left_wheel: Wheel,
    pub right_wheel: Wheel,

    pub steering: SteeringWheel,

    pub left_light: LightSensor,
    pub right_light: LightSensor,

    pub remote_control_receiver: RemoteControlReceiver,
}

impl ElectricDog {
    /// Instantiate and setup the robot.
    ///
    /// It creates and setup all modules. It connect each module to its dedicated pin.
    ///
    /// ## Examples
    ///
    /// ```
    /// let dog = electric_dog::ElectricDog::new();
    /// ```
    ///
    pub fn new() -> ElectricDog {
        ElectricDog {
            left_wheel: Wheel::new(gpio::Pin::P1),
            right_wheel: Wheel::new(gpio::Pin::P2),
            steering: SteeringWheel::new(gpio::Pin::P8),
            left_light: LightSensor::new(gpio::Pin::P12),
            right_light: LightSensor::new(gpio::Pin::P13),
            remote_control_receiver: RemoteControlReceiver::new(gpio::Pin::P9),
        }
    }
    /// Make the robot move forward.
    ///
    /// The robot will start moving forward until stopped.
    ///
    pub fn move_forward(&mut self) {
        self._move(&Steering::Center);
    }
    /// Make the robot turn left.
    ///
    /// The robot will start turning left until stopped.
    ///
    pub fn turn_left(&mut self) {
        self._move(&Steering::Left);
    }
    /// Make the robot turn right.
    ///
    /// The robot will start turning right until stopped.
    ///
    pub fn turn_right(&mut self) {
        self._move(&Steering::Right);
    }
    /// Stop the robot.
    pub fn stop(&mut self) {
        self.left_wheel.off();
        self.right_wheel.off();
    }

    fn _move(&mut self, steering: &Steering) {
        self.left_wheel.on();
        self.right_wheel.on();

        self.steering.set(steering);
    }

    /// Give the light sensor status.
    ///
    /// It can be:
    ///
    /// * Both (if both left and right light sensor are detecting something)
    /// * Left (only the left sensor is detecting something)
    /// * Right (only the right sensor is detecting something)
    /// * None (none of the sensors are detecting anything)
    ///
    /// ## Examples
    ///
    /// ```
    /// use electric_dog::module::Lights;
    ///
    /// let mut dog = electric_dog::ElectricDog::new();
    ///
    /// match dog.lights() {
    ///     Lights::Both => dog.move_forward(),
    ///     Lights::Left => dog.turn_left(),
    ///     Lights::Right => dog.turn_right(),
    ///     Lights::None => dog.stop(),
    /// }
    /// ```
    pub fn lights(&self) -> Lights {
        let on_left = self.left_light.detect();
        let on_right = self.right_light.detect();

        match (on_left, on_right) {
            (true, true) => Lights::Both,
            (true, false) => Lights::Left,
            (false, true) => Lights::Right,
            (false, false) => Lights::None,
        }
    }
}
