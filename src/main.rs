#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate alloc;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use mini_os::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    println,
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting Kernel.....");
    mini_os::init();

    let phys_frame_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_frame_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    crate::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Failed to init heap");

    let x = Box::new(77);
    println!("Value @ {:?}", x);

    #[cfg(test)]
    test_main();

    println!("BLAHHH");

    mini_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mini_os::hlt_loop();
}
