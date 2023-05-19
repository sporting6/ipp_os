#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ipp_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use ipp_os::{println, hlt_loop};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::{Page}, VirtAddr};
    use ipp_os::memory;

    println!("Hello World{}", "!");
    ipp_os::init();

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