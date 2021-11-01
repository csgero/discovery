#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};
use heapless::Vec;

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

struct SerialPort<'a> {
    usart1: &'a usart1::RegisterBlock,
}

impl fmt::Write for SerialPort<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            send_byte(self.usart1, b);
        }
        Ok(())
    }
}

fn send_byte(usart1: &usart1::RegisterBlock, b: u8) {
    while usart1.isr.read().txe().bit_is_clear() {}
    usart1.tdr.write(|w| w.tdr().bits(u16::from(b)));
}

fn receive_byte(usart1: &usart1::RegisterBlock) -> u8 {
    while usart1.isr.read().rxne().bit_is_clear() {}
    usart1.rdr.read().rdr().bits() as u8
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();
    let mut serial = SerialPort { usart1 };

    uprintln!(serial, "Reversing...");

    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            let byte_in = receive_byte(usart1);
            if byte_in != b'\r' {
                if buffer.push(byte_in).is_err() {
                    uprintln!(serial, "Error: buffer full");
                    break;
                }
            } else {
                while let Some(byte_out) = buffer.pop() {
                    send_byte(usart1, byte_out);
                }
                send_byte(usart1, b'\n');
                send_byte(usart1, b'\r');
                break;
            }
        }
    }
}
