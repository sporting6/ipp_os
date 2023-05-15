#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ipp_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use ipp_os::{println, print, hlt_loop};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    ipp_os::init();
    
    let ptr = 0x2031b4 as *mut u32;

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

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