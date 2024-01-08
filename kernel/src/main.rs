#![no_std]
#![no_main]

extern crate alloc;

use bootloader_api::info::MemoryRegionKind;
use bootloader_api::{entry_point, BootInfo};
use core::arch::asm;
use humansize::{format_size, format_size_i, FormatSize, BINARY};
use itertools::Itertools;
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

    print_mem_info(boot_info);

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();
}

fn print_mem_info(boot_info: &BootInfo) {
    let mut mem_uses = [0; 3];
    for mem in &*boot_info.memory_regions {
        let index = match mem.kind {
            MemoryRegionKind::Usable => 2,
            MemoryRegionKind::Bootloader => 1,
            MemoryRegionKind::UnknownUefi(_) => 0,
            MemoryRegionKind::UnknownBios(_) => 0,
            _ => 0,
        };
        mem_uses[index] += mem.end - mem.start;
    }
    info!(
        "memory:{:?}",
        ["unknown", "bootloader", "usable"]
            .into_iter()
            .zip(mem_uses.iter().map(|s| s.format_size(BINARY)))
            .format(",")
    );
}
