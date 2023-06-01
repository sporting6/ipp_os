#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ipp_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use alloc::string::String;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ipp_os::{
    allocator, hlt_loop,
    memory::{self, BootInfoFrameAllocator},
    println,
    vga_buffer::{cursor::CursorTrait, VGABuffer, BUFFER_WIDTH, WRITER},
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    ipp_os::init();
    println!("Loading Shell....");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    WRITER.lock().cursor.enable(0, 24);
    WRITER.lock().clear();
    WRITER.lock().cursor.update();

    #[cfg(test)]
    test_main();

    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ipp_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ipp_os::test_panic_handler(info)
}
