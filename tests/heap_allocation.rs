#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};

use myos::{self, heap::HEAP_SIZE, hlt_loop, panic_handler};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler::test_panic_handler(info)
}

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    myos::init(boot_info);

    test_main();

    hlt_loop()
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(1);
    let heap_value_2 = Box::new(2);

    assert_eq!(*heap_value_1, 1);
    assert_eq!(*heap_value_2, 2);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);

        assert_eq!(*x, i);
    }
}
