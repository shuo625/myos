#![no_std]
#![no_main]

mod vga_buffer;
mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World! numbers are {} and {}", 42, 1.0 / 3.0);
    panic!("Some panic message");

    #[allow(unreachable_code)]
    loop {}
}
