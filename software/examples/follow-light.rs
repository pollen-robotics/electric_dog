#![feature(used)]
#![no_std]

#[cfg(target_arch = "arm")]
extern crate cortex_m_rt;
#[cfg(target_arch = "arm")]
extern crate cortex_m;
#[cfg(target_arch = "arm")]
use cortex_m::asm;

extern crate electric_dog;

use electric_dog::ElectricDog;
use electric_dog::module::Lights;

fn main() {
    let mut electric_dog = ElectricDog::new();

    loop {
        match electric_dog.lights() {
            Lights::Both => electric_dog.move_forward(),
            Lights::Left => electric_dog.turn_left(),
            Lights::Right => electric_dog.turn_right(),
            Lights::None => electric_dog.stop(),
        }
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
