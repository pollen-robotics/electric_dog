extern crate luos;
extern crate electric_dog;

use electric_dog::module::{Wheel, Eye};

fn main() {
    let luos_core = luos::Core::new();

    let left_wheel = Wheel::new();
    let right_wheel = Wheel::new();
    luos_core.register(&left_wheel);
    luos_core.register(&right_wheel);

    let left_eye = Eye::new();
    let right_eye = Eye::new();
    luos_core.register(&left_eye);
    luos_core.register(&right_eye);

    loop {
        if left_eye.detect() {
            left_wheel.on();
        } else {
            left_wheel.off();
        }

        if right_eye.detect() {
            right_wheel.on();
        } else {
            right_wheel.off();
        }
    }
}
