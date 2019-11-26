#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};
use heapless::{consts, Vec};

// Maintainers solution
#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            // Wait until there is data available
            while usart1.isr.read().rxne().bit_is_clear() {}

            // Retrieve the data
            let byte = usart1.rdr.read().rdr().bits() as u8;

            // Push the byte into our buffer. When doing so, if
            // the buffer is full -> return error message
            if buffer.push(byte).is_err() {
                // Buffer is full, write error back to usart
                for byte in b"error: buffer full \n\r" {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }

            // If the byte is a carriage return, iterate over our buffer
            // in reverse and write it back to the usart
            if byte == 13 {
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }
        }
    }
}
