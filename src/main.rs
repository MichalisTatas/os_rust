#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

#[cfg(test)]
use crate::custom_test::test_runner;

#[cfg(test)]
mod custom_test;

mod vga_buffer;

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello from main{}", "!");

    #[cfg(test)]
    test_main();

    // panic!("This is a test panic!");

    loop{}
}