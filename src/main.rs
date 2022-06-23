#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};

use alloc::boxed::Box;
use core::panic::PanicInfo;

use myos::{
    hlt_loop, panic_handler, println,
    task::{executor::SimpleExecutor, keyboard, Task},
};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_handler::panic_handler(info);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_handler::test_panic_handler(info);
}

entry_point!(main);

#[allow(unused_variables)]
fn main(boot_info: &'static BootInfo) -> ! {
    #[cfg(not(test))]
    kernel_main(boot_info);

    #[cfg(test)]
    test_main();

    hlt_loop();
}

#[allow(dead_code)]
fn kernel_main(boot_info: &'static BootInfo) {
    println!("Hello World! numbers are {} and {}", 42, 1.0 / 3.0);

    myos::init(boot_info);

    let heap_value = Box::new(233);

    println!("I am at heap {}", heap_value);

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    println!("I did not crash!");
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number is {}", number);
}
