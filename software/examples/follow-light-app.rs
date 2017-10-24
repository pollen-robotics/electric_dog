extern crate electric_dog;

use electric_dog::ElectricDog;

use electric_dog::app::{App, FollowLight, Logger};
use electric_dog::module::AppSelectorPosition;

fn main() {
    let mut electric_dog = ElectricDog::new();

    let mut logger = Logger::new();
    let mut follow_light = FollowLight::new();

    loop {
        match electric_dog.app_selector.state() {
            AppSelectorPosition::Left => {
                follow_light.update(&mut electric_dog);
            }
            _ => {}
        }

        logger.update(&mut electric_dog);
    }
}
