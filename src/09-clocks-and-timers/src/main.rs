#![no_main]
#![no_std]

use aux9::{entry, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    tim6.arr.write(|w| w.arr().bits(ms));

    // start the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // wait for update event
    while !tim6.sr.read().uif().bit_is_set() {}

    // reset interrupt flag
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9::init();

    // initialize TIM6
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    // enable one pulse, disable counter
    tim6.cr1.write(|w| {
        w.opm().set_bit();
        w.cen().clear_bit()
    });

    // set timer to 1khz
    tim6.psc.write(|w| w.psc().bits(7_999));

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
