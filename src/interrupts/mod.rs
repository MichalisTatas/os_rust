use crate::println;

mod idt;

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, divide_by_zero_wrapper);

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

// use naked function wrapper because otherwise
// we have the prologue of the function first 
// and reading rsp gives us junk
#[unsafe(naked)]
extern "C" fn divide_by_zero_wrapper() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "mov rdi, rsp",                // Move stack pointer to rdi
            "call {handler}",              // Call Rust handler function
            handler = sym divide_by_zero_handler,
        )
    }
}

extern "C" fn divide_by_zero_handler(stack_frame: ExceptionStackFrame) -> ! {
    println!("{:#?}", stack_frame);

    println!("EXCEPTION: DIVIDE BY ZERO !!");

    loop{}
}

pub fn init() {
    IDT.load();
}