#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[allow(unused_imports)]
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

use memory::BootInfoFrameAllocator;

pub mod exit;
pub mod gdt;
pub mod heap;
pub mod interrupts;
pub mod memory;
pub mod panic_handler;
pub mod serial;
pub mod task;
pub mod test_runner;
pub mod utils;
pub mod vga_buffer;

pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    heap::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}

#[cfg(test)]
entry_point!(main);

/// entry point for cargo test
#[cfg(test)]
#[no_mangle]
fn main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

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
