use hal::gpio;

pub struct RemoteControlReceiver {
    pin: gpio::Pin,
}

impl RemoteControlReceiver {
    pub fn new(pin: gpio::Pin) -> RemoteControlReceiver {
        RemoteControlReceiver { pin }
    }
    pub fn triggered(&self) -> bool {
        gpio::read(&self.pin)
    }
}
