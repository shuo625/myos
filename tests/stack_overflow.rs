#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use core::panic::PanicInfo;
use myos::panic_handler;
use myos::{serial_print, serial_println};
use myos::exit;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_handler::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    myos::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    // prevent tail recursion optimization
    volatile::Volatile::new(0).read();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable  = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
               .set_handler_fn(test_double_fault_handler)
               .set_stack_index(myos::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64
) -> ! {
    serial_println!("[OK]");
    exit::exit_qemu(exit::QemuExitCode::Success);

    loop {}
}