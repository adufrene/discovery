#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1::RegisterBlock};
use heapless::{consts, Vec};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort<'a> {
    usart1: &'a mut RegisterBlock,
}

impl <'a> fmt::Write for SerialPort<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            // wait until we can send
            while self.usart1.isr.read().txe().bit_is_clear() { }

            unsafe { self.usart1.tdr.write(|w| w.tdr().bits(u16::from(byte))); }
        }

        // TODO
        Ok(())
    }
}

/*
#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut _itm) = aux11::init();

    let mut serial = SerialPort { usart1 };

    uprintln!(serial, "The answer is {}", 40 + 2);

    loop {}
}

*/


/*
#[entry]
fn main() -> ! {
    let (usart1, _, _) = aux11::init();

    loop {
        // wait for data
        while usart1.isr.read().rxne().bit_is_clear() {}

        let incoming_data = usart1.rdr.read().bits() as u8;

        // wait for output to be ready
        while usart1.isr.read().txe().bit_is_clear() {}

        usart1
            .tdr
            .write(|w| unsafe { w.tdr().bits(u16::from(incoming_data)) });
    }
}
*/

#[entry]
fn main() -> ! {
    let (usart1, _, _) = aux11::init();

    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        // wait for data
        while usart1.isr.read().rxne().bit_is_clear() { }


        let incoming_byte = usart1.rdr.read().bits() as u8;

        if incoming_byte == 13 {
            send_reversed_buffer(usart1, &buffer);
            buffer.clear();
        } else {
            if let Err(e) =  buffer.push(incoming_byte) {
                uprintln!(SerialPort { usart1 }, "An error occurred: {}", e);
            }
        }

    }
}

fn send_reversed_buffer(usart: &mut RegisterBlock, buffer: &Vec<u8, consts::U32>) {
    for byte in buffer.iter().rev() {
        // wait for txn to be ready
        while usart.isr.read().txe().bit_is_clear() { }

        usart.tdr.write(|w| unsafe { w.tdr().bits(u16::from(*byte)) });
    }
}


