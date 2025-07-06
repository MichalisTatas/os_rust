/*
    Put the tests that are supposed to panic here. Can not use the 
    #[should_panic] attribute that allows to construct tests that
    fail since it is not supported in [no_std] environments.

    With the present approach only a single test can be run.
*/

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{default, panic::PanicInfo};
use os_rust::{QemuExitCode, exit_qemu, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop{}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
        serial_println!("[Test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }

    exit_qemu(crate::QemuExitCode::Success);
}

#[test_case]
fn should_fail() {
    serial_println!("should_panic::should_fail\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[Passed]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}