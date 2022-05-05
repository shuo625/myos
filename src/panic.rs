use core::panic::PanicInfo;

#[cfg(not(test))]
use crate::println;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
use crate::exit;
#[cfg(test)]
use crate::serial_println;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit::exit_qemu(exit::QemuExitCode::Failed);

    loop {}
}