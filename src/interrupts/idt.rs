use x86_64::instructions::segmentation;
use x86_64::structures::idt::EntryOptions;
use x86_64::structures::SegmentSelector;
use x86_64::PrivilegeLevel;

/*
    idt can have up to 256 entries 
    but we only need 16 at the moment for our kernel
*/
 
pub struct Idt([Entry; 16]);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    pointer_low:    u16,
    gdt_selector:   SegmentSelector,
    options:        EntryOptions,
    pointer_middle: u16,
    pointer_high:   u32,
    reserved:       u32,
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions