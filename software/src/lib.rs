extern crate luos;

extern crate mockup_hal as hal;
use hal::gpio;

pub mod module;

use module::Wheel;
use module::{Steering, SteeringWheel};
use module::{LightSensor, Lights};
use module::RemoteControl;


pub struct ElectricDog {
    pub left_wheel: Wheel,
    pub right_wheel: Wheel,

    pub steering: SteeringWheel,

    pub left_light: LightSensor,
    pub right_light: LightSensor,

    pub remote_control: RemoteControl,
}

impl ElectricDog {
    pub fn new() -> ElectricDog {
        ElectricDog {
            left_wheel: Wheel {},
            right_wheel: Wheel {},
            steering: SteeringWheel {},
            left_light: LightSensor::new(gpio::Pin::P12),
            right_light: LightSensor::new(gpio::Pin::P13),
            remote_control: RemoteControl::new(gpio::Pin::P9),
        }
    }

    pub fn move_forward(&mut self) {
        self._move(&Steering::Center);
    }
    pub fn turn_left(&mut self) {
        self._move(&Steering::Left);
    }
    pub fn turn_right(&mut self) {
        self._move(&Steering::Right);
    }
    pub fn stop(&mut self) {
        self.left_wheel.off();
        self.right_wheel.off();
    }

    fn _move(&mut self, steering: &Steering) {
        self.left_wheel.on();
        self.right_wheel.on();

        self.steering.set(steering);
    }

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
