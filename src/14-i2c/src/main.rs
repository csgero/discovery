#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const WHO_AM_I: u8 = 0x4F;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer

    i2c1.cr2.write(|w| {
        w.add10().clear_bit();
        w.rd_wrn().write();
        w.sadd().bits(MAGNETOMETER);
        w.nbytes().bits(1);
        w.autoend().clear_bit();
        w.start().start()
    });

    while i2c1.isr.read().txis().bit_is_clear() {}
    i2c1.txdr.write(|w| w.txdata().bits(WHO_AM_I));

    while i2c1.isr.read().tc().bit_is_clear() {}

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        i2c1.cr2.write(|w| {
            w.rd_wrn().read();
            w.sadd().bits(MAGNETOMETER);
            w.nbytes().bits(1);
            w.autoend().set_bit();
            w.start().start()
        });

        while i2c1.isr.read().rxne().bit_is_clear() {}
        i2c1.rxdr.read().bits()
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", WHO_AM_I, byte);

    loop {}
}
