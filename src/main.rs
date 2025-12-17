#![no_std]
#![no_main]

use core::panic::PanicInfo;

use mini_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    mini_os::init();
    // mini_os::custom_init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("BLAHHH");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
