use core::num;

use x86_64::structures::paging::Page;

use crate::println;

mod idt;

// use naked function wrapper because otherwise
// we have the prologue of the function first 
// and reading rsp gives us junk
macro_rules!  handler{
    ($name: ident) => {{
        #[unsafe(naked)]
        extern "C" fn naked_wrapper() -> ! {
            unsafe {
                core::arch::naked_asm!(
                "mov rdi, rsp",                // Move stack pointer to rdi
                "sub rsp, 8",                  // align the stack pointer
                "call {handler}",              // Call Rust handler function
                handler = sym $name,
                )   
            } 
        }    
        naked_wrapper    
    }}
}

// use naked function wrapper because otherwise
// we have the prologue of the function first 
// and reading rsp gives us junk
macro_rules!  handler_with_error_code{
    ($name: ident) => {{
        #[unsafe(naked)]
        extern "C" fn naked_wrapper() -> ! {
            unsafe {
                core::arch::naked_asm!(
                "pop rsi",                     // pop error code into rsi
                "mov rdi, rsp",                // Move stack pointer to rdi
                "sub rsp, 8",                  // align the stack pointer
                "call {handler}",              // Call Rust handler function
                handler = sym $name,
                )   
            } 
        }    
        naked_wrapper    
    }}
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_handler));
        idt
    };
}

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

struct PageFaultErrorCode{
    // when set, page fault was caused by a page-protection violation. When not-set, it was caused by a non-present page
    PRESENT: bool,
    
    // When set, the page fault was caused by a write access. When not set, it was caused by a read access.
    WRITE: bool,
    
    // When set, the page fault was caused while CPL = 3. This does not necessarily mean that the page fault was a privilege violation.
    USER: bool,
    
    // When set, one or more page directory entries contain reserved bits which are set to 1. This only applies when the PSE or PAE flags in CR4 are set to 1.
    RESERVED_WRITE: bool,
    
    // When set, the page fault was caused by an instruction fetch. This only applies when the No-Execute bit is supported and enabled.
    INSTRUCTION_FETCH: bool,
}

impl PageFaultErrorCode {
    pub fn init(number: u64) -> Self {
        let mut temp: PageFaultErrorCode = PageFaultErrorCode{
            PRESENT:           (number & (1 << 0)) != 0,
            WRITE:             (number & (1 << 1)) != 0,
            USER:              (number & (1 << 2)) != 0,
            RESERVED_WRITE:    (number & (1 << 3)) != 0,
            INSTRUCTION_FETCH: (number & (1 << 4)) != 0,
        };
        temp
    }

    pub fn print_pagefault_errorcode(&self) -> () {
        if self.PRESENT {
            println!("PAGE FAULT: Page Protection Violation");
        }
        else {
            println!("PAGE FAULT: Non Present Page");
        }

        if self.WRITE {
            println!("PAGE FAULT: Write Access");
        }
        else {
            println!("PAGE FAULT: Read Access");
        }

        if self.USER {
            println!("PAGE FAULT: CPL 3");
        }
        else {
            println!("PAGE FAULT: CPL 1 OR 2");
        }

        if self.RESERVED_WRITE {
            println!("PAGE FAULT: Reserved bits are set to 1");
        }

        if self.INSTRUCTION_FETCH {
            println!("PAGE FAULT: Instruction fetch");
        }
    }
}


extern "C" fn divide_by_zero_handler(stack_frame: ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame );

    loop{}
}

extern  "C" fn invalid_opcode_handler(stack_frame: ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame);

    loop {}
}

extern "C" fn page_fault_handler(stack_frame: ExceptionStackFrame, error_code: u64) -> ! {
    let code: PageFaultErrorCode = PageFaultErrorCode::init(error_code);

    println!("\nEXCEPTION: PAGE FAULT  with error code {:?}\n {:?}\n {:#?}",
        error_code, code.print_pagefault_errorcode(), stack_frame);

    loop{}
}

pub fn init() {
    IDT.load();
}