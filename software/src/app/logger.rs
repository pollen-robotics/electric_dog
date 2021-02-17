use ElectricDog;
use app::App;

pub struct Logger {}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }
}

impl App for Logger {
    fn update(&mut self, _dog: &mut ElectricDog) {
        // Log new state somewhere...
    }
}
