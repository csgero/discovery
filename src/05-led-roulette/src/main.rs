#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    
    let half_period = 50_u16;

    let mut curr: usize = 0;
    let mut prev: usize = 7;
    let mut next: usize = 1;

    leds[0].on().ok();
    leds[1].on().ok();
    
    loop {
        delay.delay_ms(half_period);
        leds[prev].off().ok();
        delay.delay_ms(half_period);
        leds[next].on().ok(); 
        
        prev = if curr == 0 {7} else {curr - 1};
        next = if curr == 7 {0} else {curr + 1};
        curr = next;
    }
}
