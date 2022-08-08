#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::testable::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ros::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::panic_handler::test_panic(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
