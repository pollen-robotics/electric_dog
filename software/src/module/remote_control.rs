use luos;
use hal::gpio;

/// Remote Control Receiver for emergency stop.
///
/// This module only check if a remote control signal is triggered.
///
/// This is used for Electric Dog as a safety procedure. The robot will only move when the Remote Control is triggered.
///
pub struct RemoteControlReceiver {
    name: &'static str,
    pin: gpio::Pin,
}

impl RemoteControlReceiver {
    /// Setup and attach the Remote Control Receiver to a `Pin`
    pub fn new(name: &'static str, pin: gpio::Pin) -> RemoteControlReceiver {
        RemoteControlReceiver { name, pin }
    }
    /// Check if the Remote Control is triggered
    pub fn triggered(&self) -> bool {
        gpio::read(&self.pin)
    }
}

impl luos::Module for RemoteControlReceiver {
    fn alias(&self) -> &'static str {
        self.name
    }
}
