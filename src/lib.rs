#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
pub mod panic_handler;
pub mod exit;
pub mod serial;
pub mod test_runner;

/// entry point for cargo test
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}