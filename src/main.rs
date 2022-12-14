#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::testable::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ros::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello!");

    ros::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ros::hlt_loop();

}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ros::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::panic_handler::test_panic(info)

}
