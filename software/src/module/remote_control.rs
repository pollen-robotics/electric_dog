use hal::gpio;

/// Remote Control Receiver for emergency stop.
///
/// This module only check if a remote control signal is triggered.
///
/// This is used for Electric Dog as a safety procedure. The robot will only move when the Remote Control is triggered.
///
pub struct RemoteControlReceiver {
    pin: gpio::Pin,
}

impl RemoteControlReceiver {
    /// Setup and attach the Remote Control Receiver to a `Pin`
    pub fn new(pin: gpio::Pin) -> RemoteControlReceiver {
        RemoteControlReceiver { pin }
    }
    /// Check if the Remote Control is triggered
    pub fn triggered(&self) -> bool {
        gpio::read(&self.pin)
    }
}
