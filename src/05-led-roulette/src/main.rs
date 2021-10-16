#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    
    let period = 50_u16;

    let mut curr: usize = 0;

    leds[0].on().ok();
    leds[1].on().ok();
    
    loop {
        let next = (curr + 1) % 8;
        leds[next].on().ok();
        delay.delay_ms(period);
        leds[curr].off().ok();
        delay.delay_ms(period);
        curr = next; 
    }
}
