#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Replaces panic since stdlibs aren't accessible
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Entry point replacing main
    println!("Welcome!");
    println!("This is Oxidized OS.");
    loop {}
}