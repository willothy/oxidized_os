#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Replaces panic since stdlibs aren't accessible
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Entry point replacing main
    loop {}
}