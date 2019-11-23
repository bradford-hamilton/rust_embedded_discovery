#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

    // Here we use volatile operations instead of plain reads/writes (explantion below main):
    unsafe {
        // A magical address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
    }


    loop {}
}

// Below was the original code... Although it does work, when running with --release flag
// the LLVM backend ends up misoptimizing and doesn't know GPIOE_BSRR is a register.
// Instead of doing 4 bitshift operations on it, it misoptimizes to one.

// unsafe {
//     // A magic address!
//     const GPIOE_BSRR: u32 = 0x48001018;

//     // Turn on the "North" LED (red)
//     *(GPIOE_BSRR as *mut u32) = 1 << 9;

//     // Turn on the "East" LED (green)
//     *(GPIOE_BSRR as *mut u32) = 1 << 11;

//     // Turn off the "North" LED
//     *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

//     // Turn off the "East" LED
//     *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
// }
