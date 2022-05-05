use crate::serial_println;
use crate::exit;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    exit::exit_qemu(exit::QemuExitCode::Success);
}