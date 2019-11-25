#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};
use heapless::{consts, Vec};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();

        // TODO Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.

        // TODO Send back the reversed string
    }
}
