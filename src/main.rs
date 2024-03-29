#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_kernel_main"]
extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use kernel::println;

/// The version of the operating sys.
pub const OS_VERSION: &str = env!("CARGO_PKG_VERSION");

entry_point!(kernel_main);

/// The kernel main function.
///
/// # Arguments
///
/// * `boot_info` - A reference to the boot information.
///
/// # Returns
///
/// * `!` - Never.
#[allow(clippy::expect_used)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    let mut executor = match kernel::init::start_kernel(boot_info) {
        Ok(executor) => executor,
        Err(why) => {
            println!("[ERROR]: Failed to initialize kernel: {err:#?}", err = why);
            kernel::hlt_loop();
        }
    };

    println!("[INFO]: Rust OS v{OS_VERSION} initialized successfully!");

    executor.run();
}

/// This function is called on panic.
///
/// # Arguments
///
/// * `info` - A reference to the panic info.
///
/// # Returns
///
/// * `!` - Never.
#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("[ERROR]: {info}");

    kernel::hlt_loop();
}

/// This function is called on panic.
///
/// # Arguments
///
/// * `info` - A reference to the panic info.
///
/// # Returns
///
/// * `!` - Never.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
