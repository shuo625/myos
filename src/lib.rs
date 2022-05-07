#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod interrupts;
pub mod vga_buffer;
pub mod panic_handler;
pub mod exit;
pub mod serial;
pub mod test_runner;

pub fn init() {
    interrupts::init_idt();
}

/// entry point for cargo test
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler::test_panic_handler(info);
}