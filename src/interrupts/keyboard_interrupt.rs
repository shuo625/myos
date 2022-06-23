use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

use crate::{
    interrupts::{InterruptIndex, PICS},
    task::keyboard,
};

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
