use ElectricDog;
use app::App;

pub struct Logger {}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }
}

impl App for Logger {
    fn update(&mut self, dog: &mut ElectricDog) {
        println!(
            "Detected lights: {:?} Moving: {:?}",
            dog.lights(),
            dog.current_move
        );
    }
}
