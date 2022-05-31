#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    hlt_loop();
}

fn main() {
    println!("Hello World! numbers are {} and {}", 42, 1.0 / 3.0);

    myos::init();

    // test page fault
    let ptr = 0xdeadbeef as *mut u32;
    unsafe {
        *ptr = 42;
    }

    println!("I did not crash!");
}
