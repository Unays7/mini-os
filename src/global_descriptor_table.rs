use lazy_static::lazy_static;
use x86_64::{
    VirtAddr,
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable},
        tss::TaskStateSegment,
    },
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let start = VirtAddr::from_ptr(&raw const STACK);
            start + STACK_SIZE
        };
        tss
    };
}

lazy_static! {
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment());
        gdt.add_entry(Descriptor::tss_segment(&TSS));
        gdt
    };
}

pub fn init() {
    GDT.load();
}

pub struct CustomGlobalDescriptorTable {} // TO-DO
