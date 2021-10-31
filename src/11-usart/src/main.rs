#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

fn send_byte(usart1: &usart1::RegisterBlock, b: u8)  {
    while !usart1.isr.read().txe().bit_is_set() {};

    usart1
        .tdr
        .write(|w| w.tdr().bits(u16::from(b)));

}

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let instant = mono_timer.now();
    for b in b"The quick brown fox jumps over the lazy fox" {
        send_byte(usart1, *b);
    };
    let elapsed = instant.elapsed();

    iprintln!(
        &mut itm.stim[0],
        "`for` loop took {} ticks ({} us)",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    loop {}
}
