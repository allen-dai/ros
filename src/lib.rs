#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testable::runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
pub mod interrupts;
pub mod panic_handler;
pub mod qemu;
pub mod serial;
pub mod testable;
pub mod vga_buffer;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::panic_handler::test_panic(info)
}

pub fn init() {
    interrupts::init_idt();
}
