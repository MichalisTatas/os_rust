use x86_64::instructions::segmentation;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::idt::HandlerFunc;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn minimal() -> Self {
        let mut options: u16 = 0;
        // bits 9-10-11 must be one
        // bit  12 must be zero
        let mask: u16 = 7 << 9; // 7 is 111 shifted by 9
        options |= mask;
        EntryOptions(options)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    // if a handler is present
    // bit 15
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        if present {
            let mask: u16 = 1 << 15;
            self.0 |= mask;
        }
        self
    }

    // 0 -> interrupts disabled
    // 1 -> interrupts enabled
    // bit 8
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        if disable {
            let mask: u16 = 1 << 8;
            self.0 &= !mask;  // clear the bit 8
        }
        self
    }

    // minimum priviledge level needed to call this handler
    // bits 13-14
    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0 |= dpl;
        self
    }

    // 0 -> dont switch stacks
    // 1-7 -> switch to the n-th interrupt stack table
    // bits 0-1-2
    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0 |= index;
        self
    }
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: extern "C" fn() -> !) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector: gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
        }        
    }

    // non present interrupt page table entry
    // gdt or pointer doesnt matter as long as present bit not set
    fn missing() -> Self {
        Entry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
        }
    }
}

impl Idt {
    pub fn new() -> Idt {
        Idt([Entry::missing(); 16])
    }

    pub fn set_handler(&mut self, entry: u8, handler: extern "C" fn() -> !) {
        self.0[entry as usize] = Entry::new(segmentation::cs(), handler);
    }

    pub fn load(&self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lidt};
        use core::mem::size_of;
        use x86_64::VirtAddr;

        let lidt_ptr = DescriptorTablePointer {
            limit: (size_of::<Self>() - 1) as u16,
            base: VirtAddr::from_ptr(self.0.as_ptr()),
        };

        unsafe {
            lidt(&lidt_ptr)
        };
    }
}


/*   IDT testing   */

#[test_case]
fn test_entry_options_minimal() {
    assert_eq!(EntryOptions(0b0000_1110_0000_0000), EntryOptions::minimal());
}

#[test_case]
fn test_entry_options_new() {
    assert_eq!(EntryOptions(0b1000_1110_0000_0000), EntryOptions::new());
}