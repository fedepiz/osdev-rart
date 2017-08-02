#![feature(const_fn)]
#![feature(alloc)]
#![feature(allocator_internals)]
#![feature(allocator_api)]
#![no_std]
#![default_lib_allocator]

use spin::Mutex;
use linked_list_allocator::Heap;

extern crate spin;
extern crate linked_list_allocator;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate alloc;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

lazy_static ! {
    static ref HEAP: Mutex<Heap> = Mutex::new(unsafe {
        Heap::new(HEAP_START, HEAP_SIZE)
    });
}

use alloc::allocator::Layout;

#[no_mangle]
pub unsafe extern fn __rdl_alloc(size: usize,
                                 align: usize,
                                 err: *mut u8) -> *mut u8 {
    let layout = Layout::from_size_align(size, align).expect("Invalid layout");
    HEAP.lock().allocate_first_fit(layout).expect("out of memory")
}
#[no_mangle]
pub unsafe extern fn __rdl_oom(err: *const u8) -> ! {
   panic!("OOM");
}

#[no_mangle]
pub unsafe extern fn __rdl_dealloc(ptr: *mut u8,
                                   size: usize,
                                   align: usize) {
    let layout = Layout::from_size_align(size, align).expect("Invalid layout");
    HEAP.lock().deallocate(ptr, layout);
}

#[no_mangle]
pub unsafe extern fn __rdl_realloc(ptr: *mut u8,
                                       old_size: usize,
                                       old_align: usize,
                                       new_size: usize,
                                       new_align: usize,
                                       err: *mut u8) -> *mut u8 {
    use core::cmp;
    let old_layout = Layout::from_size_align(old_size, old_align).expect("Invalid layout");
    let new_layout = Layout::from_size_align(new_size, new_align).expect("Invalid layout");
    let new_ptr = HEAP.lock().allocate_first_fit(new_layout).expect("Out of memory");
    core::ptr::copy(ptr, new_ptr, cmp::min(old_size, new_size));
    HEAP.lock().deallocate(ptr, old_layout);
    new_ptr
}
