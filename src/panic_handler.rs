use core::panic::PanicInfo;

use crate::exit;
use crate::{hlt_loop, println, serial_println};

pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);

    hlt_loop();
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit::exit_qemu(exit::QemuExitCode::Failed);

    hlt_loop();
}
