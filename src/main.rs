#![no_std]
#![no_main]

use core::panic::PanicInfo;

use mini_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    mini_os::init();
    // mini_os::custom_init();
    // x86_64::instructions::interrupts::int3();
    #[allow(unconditional_recursion)]
    fn stackoverflow() {
        stackoverflow();
    }

    // stackoverflow();

    // unsafe {
    //     *(0xfaaadd as *mut u8) = 42;
    // }

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
