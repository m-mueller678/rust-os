#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use osos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    osos::init();
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    #[cfg(test)]
    test_main();

    x86_64::instructions::interrupts::int3();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osos::test_panic_handler(info)
}
