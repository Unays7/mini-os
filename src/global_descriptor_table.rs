use lazy_static::lazy_static;
use x86_64::structures::tss::TaskStateSegment;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // Create the stack and store the addy in the stack table
            todo!() };
        tss
    };
}

pub struct GlobalDescriptorTable {}
