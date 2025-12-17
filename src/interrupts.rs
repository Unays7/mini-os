use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", sf);
}

extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _err_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", sf);
}

extern "x86-interrupt" fn page_fault_handler(
    sf: InterruptStackFrame,
    err_code: PageFaultErrorCode,
) {
    panic!("EXCEPTION: PAGE FAULT\n {:?} ERROR:{:?}", sf, err_code)
}

/// Interrupt Tests

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
