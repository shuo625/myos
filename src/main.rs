#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

use core::panic::PanicInfo;

use myos::panic_handler;
use myos::{hlt_loop, println};

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

fn main(boot_info: &'static BootInfo) -> ! {
    #[cfg(not(test))]
    kernel_main();

    #[cfg(test)]
    test_main();

    hlt_loop();
}

fn kernel_main() {
    println!("Hello World! numbers are {} and {}", 42, 1.0 / 3.0);

    myos::init();

    use x86_64::registers::control::Cr3;
    // test page fault
    let (page_4_level_table, _) = Cr3::read();
    println!(
        "Level 4 page table at {:?}",
        page_4_level_table.start_address()
    );

    println!("I did not crash!");
}
