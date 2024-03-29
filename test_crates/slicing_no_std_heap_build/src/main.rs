#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
// TODO the following fails
//use alloc::vec;
struct DummyAllocator {}

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator {};

unsafe impl Sync for DummyAllocator {}

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unimplemented!()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    slicing_any_std_test::slices::bool_slice::construct_from_existing_data();
    //slicing_any_std_test::slices::bool_slice::new_contains_initial_false();
    //slicing_any_std_test::slices::bool_slice::from_vec_etc();
    loop {}
}
