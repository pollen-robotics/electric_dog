use hal::gpio;

pub struct RemoteControl {
    pin: gpio::Pin,
}

impl RemoteControl {
    pub fn new(pin: gpio::Pin) -> RemoteControl {
        RemoteControl { pin }
    }
    pub fn triggered(&self) -> bool {
        gpio::read(&self.pin)
    }
}
