#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use mini_os::{QemuExitCode, exit_qemu, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_println!("stack_overflow::stack_overflow...\t");

    serial_println!("Initializing GDT...");
    mini_os::global_descriptor_table::init();

    serial_println!("Loading test IDT...");
    init_test_idt();

    serial_println!("Triggering stack overflow...");
    stackoverflow();

    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[allow(unconditional_recursion)]
fn stackoverflow() {
    stackoverflow();
    volatile::Volatile::new(0).read();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(mini_os::global_descriptor_table::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
