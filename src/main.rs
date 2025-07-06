#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_rust::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello from main{}", "!");

    #[cfg(test)]
    test_main();

    // panic!("This is a test panic!");

    loop{}
}

// Function called on panic while not in test mode
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// Function called on panic while in test mode
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    os_rust::test_panic_handler(_info)
}