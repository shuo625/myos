use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

use crate::gdt;

mod pic;
mod breakpoint_exception;
mod double_fault_exception;
mod timer_interrupt;
mod keyboard_interrupt;

pub use pic::PICS;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = pic::PIC_1_OFFSET,
    Keyboard
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_exception::breakpoint_handler);
        
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_exception::double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt::timer_interrupt_handler);

        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt::keyboard_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}