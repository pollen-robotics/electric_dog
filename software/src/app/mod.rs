use ElectricDog;

mod logger;
pub use app::logger::Logger;

mod follow_light;
pub use app::follow_light::FollowLight;


pub trait App {
    fn update(&mut self, dog: &mut ElectricDog);
}
