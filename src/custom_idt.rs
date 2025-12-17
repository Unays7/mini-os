use bit_field::BitField;
use core::arch::asm;
use lazy_static::lazy_static;

use crate::println;

lazy_static! {
    static ref CustomIdt: Idt = {
        let mut idt = Idt::new();
        idt.set_handler(0, divide_by_zero_handler);
        idt
    };
}

pub fn init_idt() {
    CustomIdt.load();
}

#[inline]
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    unsafe {
        asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
    }
}

pub struct DescriptorTablePointer {
    base: u64,
    limit: u16,
}

pub type MyHandlerFunc = extern "C" fn() -> !;

extern "C" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

pub struct Idt([IdtEntry; 16]);

impl Idt {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Idt {
        Idt([IdtEntry::minimal(); 16])
    }

    pub fn load(&self) {
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }

    pub fn set_handler(&mut self, idx: u8, hanlder: MyHandlerFunc) -> &mut IdtEntryOptions {
        self.0[idx as usize] = IdtEntry::minimal();
        &mut self.0[idx as usize].options
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdtEntry {
    low_pointer: u16,
    middle_pointer: u16,
    gdt_selector: u16,
    options: IdtEntryOptions,
    high_pointer: u32,
    reserved: u32,
}

impl IdtEntry {
    const fn minimal() -> IdtEntry {
        IdtEntry {
            low_pointer: 0,
            middle_pointer: 0,
            gdt_selector: 0,
            options: IdtEntryOptions::minimal(),
            high_pointer: 0,
            reserved: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct IdtEntryOptions(u16);

impl IdtEntryOptions {
    const fn minimal() -> Self {
        Self(0b1110_0000_0000)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    #[inline]
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    #[inline]
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    #[inline]
    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0.set_bits(13..15, dpl);
        self
    }

    #[inline]
    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index);
        self
    }
}
