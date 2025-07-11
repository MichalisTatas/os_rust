#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_rust::println;
use os_rust::interrupts;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello from main{}", "!");

    #[cfg(test)]
    test_main();

    // panic!("This is a test panic!");

    // initialize IDT
    interrupts::init();

    unsafe { core::arch::asm!("ud2") };

    // divide_by_zero();

    println!("Did not crash on divide by zero exception");

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


fn divide_by_zero() {
    unsafe {
        core::arch::asm!(
            "mov dx, 0",
            "div dx",
            out("ax") _,  // clobber ax
            out("dx") _,  // clobber dx
            options(nostack, nomem, preserves_flags)
        );
    }
}