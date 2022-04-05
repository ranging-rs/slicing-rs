// Thanks to https://blog.dbrgn.ch/2019/12/24/testing-for-no-std-compatibility
// \--> ensure_no_std
#![no_std]
#![no_main]

// cargo rustc -- -C link-arg=-nostartfiles
#[allow(unused_imports)]
use ranging; // No need to include all submodules - cargo will load them all anyway.

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
