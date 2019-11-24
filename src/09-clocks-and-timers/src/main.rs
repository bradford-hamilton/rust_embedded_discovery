#![no_main]
#![no_std]

use aux9::{entry, tim6};

const CYCLES_PER_MS: u16 = 7_999;

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Set the timer to go off in `ms` ticks. 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));

    // Enable the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // Wait until the alarm goes off (until the "update event" occurs). This pattern of just waiting
    // until some condition is met, in this case that UIF becomes 1, is known as busy waiting 
    while !tim6.sr.read().uif().bit_is_set() {}

    // Clear the update event flag
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9::init();

    // Initialize TIM6.
    // Power on the TIM6 timer. This bit is in the APB1ENR register of the RCC register block.
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    // OPM select one pulse mode
    // CEN keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // Configure the prescaler to have the counter operate at 1 KHz
    tim6.psc.write(|w| w.psc().bits(CYCLES_PER_MS));
    
    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}

// First try at the timer - uses a for loop to run the necessary amount of iterations
// to take up the amount of time requested in ms as a delay

// fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
//     let loops = ms as usize * CYCLES_PER_MS;

//     for _ in 0..loops {
//         // Need this so that the compiler does not optimize the "useless" for loop away
//         aux9::nop()
//     }
// }