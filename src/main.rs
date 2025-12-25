#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use mini_os::{
    memory::{active_lvl_4_pt, next_lvl},
    println,
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting Kernel.....");
    mini_os::init();

    let offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_pt = unsafe { active_lvl_4_pt(offset) };

    for (i, entry) in l4_pt.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    let l3_table = unsafe { next_lvl(l4_pt, offset) };

    if let Some(l3) = l3_table {
        for (i, entry) in l3.iter().enumerate() {
            if !entry.is_unused() {
                println!("L3 Entry {}: {:?}", i, entry);
            }
        }
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
