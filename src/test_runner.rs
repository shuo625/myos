use crate::println;
use crate::exit;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    exit::exit_qemu(exit::QemuExitCode::Success);
}