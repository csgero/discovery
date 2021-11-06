//#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const OUT_X_L_M: u8 = 0xE8;
const CFG_REG_A_M: u8 = 0x60;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = aux14::init();

    i2c1.cr2.write(|w| {
        w.add10().clear_bit();
        w.rd_wrn().write();
        w.sadd().bits(MAGNETOMETER);
        w.nbytes().bits(2);
        w.autoend().set_bit();
        w.start().start()
    });

    // configure magnetometer for continuous output
    while i2c1.isr.read().txis().bit_is_clear() {}
    i2c1.txdr.write(|w| w.txdata().bits(CFG_REG_A_M));
    while i2c1.isr.read().txis().bit_is_clear() {}
    i2c1.txdr.write(|w| w.txdata().bits(0x0));

    loop {
        i2c1.cr2.write(|w| {
            w.rd_wrn().write();
            w.sadd().bits(MAGNETOMETER);
            w.nbytes().bits(1);
            w.autoend().clear_bit();
            w.start().start()
        });

        while i2c1.isr.read().txis().bit_is_clear() {}
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_L_M));

        while i2c1.isr.read().tc().bit_is_clear() {}

        i2c1.cr2.write(|w| {
            w.rd_wrn().read();
            w.sadd().bits(MAGNETOMETER);
            w.nbytes().bits(6);
            w.autoend().set_bit();
            w.start().start()
        });

        let mut buffer = [0i16; 3];
        let buffer_u8 = unsafe { buffer.align_to_mut::<u8>().1 };
        for byte in buffer_u8 {
            while i2c1.isr.read().rxne().bit_is_clear() {}
            *byte = i2c1.rxdr.read().rxdata().bits()
        }

        iprintln!(&mut itm.stim[0], "{:?}",buffer);

        delay.delay_ms(1_000_u16);
    }
}
