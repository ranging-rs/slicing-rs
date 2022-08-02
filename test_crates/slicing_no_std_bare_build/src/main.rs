#![no_std]
#![no_main]

use core::panic::PanicInfo;

struct DummyAllocator {}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    slicing_any_std_test::slices::bool_slice::construct_from_existing_data();
    slicing_any_std_test::slices::bool_slice::new_contains_initial_false();
    loop {}
}
