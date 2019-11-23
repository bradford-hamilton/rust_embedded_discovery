#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    loop {
        for i in 0..8 {
            if i == 7 {
                leds[0].on();
            } else if i == 8 {
                leds[1].on()
            } else {
                leds[i + 1].on();
            }

            delay.delay_ms(50_u8);
            leds[i].off();
            delay.delay_ms(50_u8);
        }
    }

    // Above is my challenge solution and below is the authors solution

    // let ms = 50_u8;
    // loop {
    //     for curr in 0..8 {
    //         let next = (curr + 1) % 8;

    //         leds[next].on();
    //         delay.delay_ms(ms);
    //         leds[curr].off();
    //         delay.delay_ms(ms);
    //     }
    // }
}
