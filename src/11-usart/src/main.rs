#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\r\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\r\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            send_byte(self.usart1, b);
        }
        Ok(())
    }
}

fn send_byte(usart1: &usart1::RegisterBlock, b: u8)  {
    while !usart1.isr.read().txe().bit_is_set() {};

    usart1
        .tdr
        .write(|w| w.tdr().bits(u16::from(b)));

}

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();
    let mut serial = SerialPort { usart1 };

    uprintln!(serial, "The answer is {}", 40 + 2);

    loop {}
}
