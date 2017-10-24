use luos;
use hal::gpio;

pub struct AppSelector {
    name: &'static str,
    pin: gpio::Input,
}

pub enum AppSelectorPosition {
    Left,
    Middle,
    Right,
}

impl AppSelector {
    pub fn new(name: &'static str, pin: gpio::Pin) -> AppSelector {
        AppSelector {
            name,
            pin: gpio::Input::setup(pin),
        }
    }
    pub fn state(&self) -> AppSelectorPosition {
        match self.pin.read() {
            true => AppSelectorPosition::Left,
            false => AppSelectorPosition::Right,
        }
    }
}

impl luos::Module for AppSelector {
    fn alias(&self) -> &'static str {
        self.name
    }
}
