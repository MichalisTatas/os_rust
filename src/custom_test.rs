use crate::{println, serial_println};

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::{exit_qemu, serial, serial_println};

    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    exit_qemu(crate::QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_println!("Trivial assertion");
    assert_eq!(1, 1);
    serial_println!("Passed");
}
