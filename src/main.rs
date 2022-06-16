#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

use core::panic::PanicInfo;

use myos::{hlt_loop, memory, panic_handler, println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_handler::panic_handler(info);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_handler::test_panic_handler(info);
}

entry_point!(main);

#[allow(unused_variables)]
fn main(boot_info: &'static BootInfo) -> ! {
    #[cfg(not(test))]
    kernel_main(boot_info);

    #[cfg(test)]
    test_main();

    hlt_loop();
}

#[allow(dead_code)]
fn kernel_main(boot_info: &'static BootInfo) {
    println!("Hello World! numbers are {} and {}", 42, 1.0 / 3.0);

    myos::init();

    println!("I did not crash!");
}
