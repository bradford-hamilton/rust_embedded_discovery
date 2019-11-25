#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};
use heapless::{consts, Vec};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes().iter() {
            // Wait until it is safe to write to TDR
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
        }
        Ok(())
    }
}

// This program writes to the TDR register. This causes the USART peripheral
// to send one byte of information through the serial interface.

// The processor can try to send bytes at a faster rate than what the hardware
// can actually handle and this results in data loss. This condition is known as
// buffer overrun. The status register (ISR) has a flag, TXE, that indicates
// if it's "safe" to write to the TDR register without incurring in data loss.
fn main() -> ! {
    let (usart1, _, _) = aux11::init();

    // Send this string from the microcontroller to minicom
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        // Wait until it is safe to write to TDR
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }

    // Use our uprintln macro to print the provided string with its stringified
    // arguments. We satisfied the fmt::Write interface for SerialPort (code above)
    // so that in theory `uprintln!` will print to minicom
    let mut serial = SerialPort { usart1 };

    uprintln!(serial, "The quick brown fox jumps {} ft high", 40 + 2);

    // Here after printing the above content from the microcontroller to minicom,
    // we are going to wait and listen for any incoming data from minicom to our
    // microcontroller.
    loop {
        // Wait until there's data available
        while serial.usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let _byte = serial.usart1.rdr.read().rdr().bits() as u8;

        aux11::bkpt();
    }
}
