#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_str("hello world").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers {} and {}", 42, 1.0 / 3.0).unwrap();

    loop {}
}
