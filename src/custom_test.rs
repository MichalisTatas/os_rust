use crate::println;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    println!("Trivial assertion");
    assert_eq!(1, 1);
    println!("Passed");
}
