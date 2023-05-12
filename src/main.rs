#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ipp_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use ipp_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    ipp_os::init();

    #[cfg(test)]
    test_main();

    println!("Did not crash, Wow!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ipp_os::test_panic_handler(info)
}