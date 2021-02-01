#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (_, gpioe) = aux7::init();

    // turn on north led
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // turn on east led
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // turn on north led
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // run on east led
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
