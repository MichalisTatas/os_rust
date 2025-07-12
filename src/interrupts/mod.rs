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
    println!("\nEXCEPTION: PAGE FAULT  with error code {:?}\n{:#?}",
        error_code, stack_frame);

    loop{}
}

pub fn init() {
    IDT.load();
}