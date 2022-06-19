#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

use myos::{hlt_loop, panic_handler, println};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler::test_panic_handler(info)
}

entry_point!(main);

fn main(_boot_info: &'static BootInfo) -> ! {
    test_main();

    hlt_loop()
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
