#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use mini_os::{
    memory::{self, BootInfoFrameAllocator},
    println,
};
use x86_64::structures::paging::{Mapper, Size4KiB};
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame},
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting Kernel.....");
    mini_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(0xDEADBEEF000));
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    unsafe {
        mapper
            .map_to(page, frame, flags, &mut frame_allocator)
            .expect("map_to failed")
            .flush();
    }

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
