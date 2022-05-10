#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use myos::{println, hlt_loop};
use myos::panic_handler;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop();
}

#[test_case]
fn test_println() {
    println!("test_println output");
}