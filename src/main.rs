#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let s = "tatas";
    println!("Hello from println {}", s);

    panic!("This is a test panic!");
    loop{}
}