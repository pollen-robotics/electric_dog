use {ElectricDog, Lights};
use app::App;

pub struct FollowLight {}

impl FollowLight {
    pub fn new() -> FollowLight {
        FollowLight {}
    }
}

impl App for FollowLight {
    fn update(&mut self, electric_dog: &mut ElectricDog) {
        if electric_dog.remote_control_receiver.triggered() {
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
