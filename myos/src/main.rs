#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use myos::println;

// Entry point for this program.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    kernel_main();

    #[cfg(test)]
    test_main();

    loop {}
}

fn kernel_main() {
    println!("Hello world!");
    myos::init();
    // x86_64::instructions::interrupts::int3();
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    println!("It did not crash!");
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
    myos::test_panic_handler(info);
    loop {}
}
