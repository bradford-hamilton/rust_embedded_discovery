#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux6::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let mut itm = aux6::init();

    // iprintln! will write to the ITM then the file we're watching
    // due to our added config lines 7 & 8 in openocd.gdb
    iprintln!(&mut itm.stim[0], "Hello, world!");
    loop {}

    // The below panic! will also write to the ITM
    // panic!("Hello, world!")
}
