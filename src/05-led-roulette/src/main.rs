#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*};

const NUM_LEDS: usize = 8;
const DELAY_DURATION: u16 = 50;


#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = aux5::init();

    loop {
        for current_led_index in 0..NUM_LEDS {
            let next_led_index = next_led(current_led_index);

            leds[next_led_index].on();
            delay.delay_ms(DELAY_DURATION);

            leds[current_led_index].off();
            delay.delay_ms(DELAY_DURATION);
        }
    }
}

fn next_led(previous_led_index: usize) -> usize {
    (previous_led_index + 1) % NUM_LEDS
}
