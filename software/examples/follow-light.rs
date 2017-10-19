extern crate electric_dog;

use electric_dog::ElectricDog;
use electric_dog::module::Lights;

fn main() {
    let mut electric_dog = ElectricDog::new();

    loop {
        if electric_dog.remote_control.triggered() {
            match electric_dog.lights() {
                Lights::Both => electric_dog.move_forward(),
                Lights::Left => electric_dog.turn_left(),
                Lights::Right => electric_dog.turn_right(),
                Lights::None => electric_dog.stop(),
            }
        } else {
            electric_dog.stop();
        }
    }
}
