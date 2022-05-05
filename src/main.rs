#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod panic;
mod exit;
#[cfg(test)]
mod test_runner;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    main();

    #[allow(unreachable_code)]
    loop {}
}

fn main() {
    println!("Hello World! numbers are {} and {}", 42, 1.0 / 3.0);
    panic!("Panic some message");
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[OK]");
}