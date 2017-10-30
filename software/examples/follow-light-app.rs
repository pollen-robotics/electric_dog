#![feature(used)]
#![no_std]

#[cfg(target_arch = "arm")]
extern crate cortex_m_semihosting;
#[cfg(target_arch = "arm")]
extern crate cortex_m_rt;
#[cfg(target_arch = "arm")]
extern crate cortex_m;
#[cfg(target_arch = "arm")]
use cortex_m_semihosting::hio;
#[cfg(target_arch = "arm")]
use core::fmt::Write;
#[cfg(target_arch = "arm")]
use cortex_m::asm;



extern crate electric_dog;

use electric_dog::ElectricDog;
use electric_dog::app::{App, FollowLight};

fn main() {
    let mut stdout = hio::hstdout().unwrap();

    let mut electric_dog = ElectricDog::new();
    let mut follow_light = FollowLight::new();

    loop {
        follow_light.update(&mut electric_dog);
        writeln!(stdout, "ok {:?}", electric_dog.left_light.get()).unwrap();
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[cfg(target_arch = "arm")]
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

#[cfg(target_arch = "arm")]
extern "C" fn default_handler() {
    asm::bkpt();
}
