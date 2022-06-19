use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

use crate::interrupts::InterruptIndex;
use crate::interrupts::PICS;
use crate::print;

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    lazy_static! {
        static ref KEYBOARD: spin::Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            spin::Mutex::new(Keyboard::new(
                layouts::Us104Key,
                ScancodeSet1,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
