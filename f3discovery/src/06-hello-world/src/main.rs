#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux6::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let mut itm = aux6::init();

    iprintln!(&mut itm.stim[0], "Hello, world!");  
    /*
    iprintln macro will format messages and output 
    them to the microntrollers ITM and (SWD) Serial Wire Debug
    */

    //there is an issue with the parsing of openocd when receiving the print statement 
    //itmdump

    loop {}
}
