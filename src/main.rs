#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ipp_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use x86_64::instructions::port::{PortGeneric, ReadWriteAccess, Port};
use core::panic::PanicInfo;
use ipp_os::vga_buffer::cursor::CursorTrait;
use ipp_os::vga_buffer::{WRITER};
use ipp_os::{allocator};
use ipp_os::{
    hlt_loop,
    memory::{self, BootInfoFrameAllocator},
    println,
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    ipp_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    WRITER.lock().cursor.enable(0, 24);

    let pos: u16 = (5* (ipp_os::vga_buffer::BUFFER_WIDTH as usize) + 5) as u16;

    let mut port1: PortGeneric<u32, ReadWriteAccess> = Port::new(0x3D4);
    let mut port2: PortGeneric<u32, ReadWriteAccess> = Port::new(0x3D5);

    unsafe {
        port1.write(0x0F as u32);
        port2.write((pos & 0xFF) as u32);
        port1.write(0x0E as u32);
        port2.write(((pos >> 8) & 0xFF) as u32);
    };

    #[cfg(test)]
    test_main();

    println!("Did not crash, Wow!");
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
