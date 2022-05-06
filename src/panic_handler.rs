use core::panic::PanicInfo;
use crate::println;
use crate::exit;
use crate::serial_println;


pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit::exit_qemu(exit::QemuExitCode::Failed);

    loop {}
}