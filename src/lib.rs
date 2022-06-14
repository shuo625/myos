#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

pub mod exit;
pub mod gdt;
pub mod interrupts;
pub mod panic_handler;
pub mod serial;
pub mod test_runner;
pub mod vga_buffer;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[cfg(test)]
entry_point!(main);

/// entry point for cargo test
#[cfg(test)]
#[no_mangle]
fn main(_boot_info: &'static BootInfo) -> ! {
    init();

    test_main();

    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler::test_panic_handler(info);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
