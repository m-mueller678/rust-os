#![no_std]
#![no_main]

extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use core::arch::asm;
use kernel::task::keyboard;
use kernel::task::{executor::Executor, Task};
use tracing::info;
use x86_64::registers;
use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::SegmentSelector;

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    info!("async number: {}", number);
}

entry_point!(kernel_main, config = &kernel::BOOT_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    unsafe {
        registers::segmentation::SS::set_reg(SegmentSelector::NULL);
        registers::segmentation::DS::set_reg(SegmentSelector::NULL);
    }

    kernel::init(boot_info);
    info!("init complete");
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();
}
