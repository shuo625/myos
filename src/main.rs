#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use myos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    #[allow(unreachable_code)]
    loop {}
}

fn main() {
    println!("Hello World! numbers are {} and {}", 42, 1.0 / 3.0);
    panic!("Panic some message");
}