#![no_std]
#![no_main]

extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use kernel::task::keyboard;
use kernel::task::{executor::Executor, Task};
use tracing::info;

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    info!("async number: {}", number);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);
    info!("init complete");
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();
}
