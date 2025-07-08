Simple OS in Rust

Future improvements ideas:
1) Use ACPI for power operations
2) TCP instead of Serial
3) Ability to run multiple tests that should fail with [no_std]
4) Use custom bootloader

# Issues
1) In impl idt cant return a reference to an unaligned field
