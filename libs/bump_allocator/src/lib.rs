#![feature(const_fn)]
#![feature(allocator_internals)]
#![no_std]
#![default_lib_allocator]

use spin::Mutex;

extern crate spin;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

static BUMP_ALLOCATOR: Mutex<BumpAllocator> =
    Mutex::new(BumpAllocator::new(HEAP_START, HEAP_SIZE));

#[derive(Debug)]
struct BumpAllocator {
    heap_start: usize,
    heap_size: usize,
    next: usize,
}

impl BumpAllocator {
    const fn new(heap_start: usize, heap_size: usize) -> BumpAllocator {
        BumpAllocator {
            heap_start: heap_start,
            heap_size: heap_size,
            next: heap_start,
        }
    }

    fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let alloc_start = align_up(self.next, align);
        let alloc_end = alloc_start.saturating_add(size);

        if alloc_end <= self.heap_start + self.heap_size {
            self.next = alloc_end;
            Some(alloc_start as *mut u8)
        } else {
            None
        }
    }
}

pub fn align_down(addr: usize, align:usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2")
    }
}

pub fn align_up(addr:usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}

#[no_mangle]
pub unsafe extern fn __rdl_alloc(size: usize,
                                 align: usize,
                                 err: *mut u8) -> *mut u8 {
    BUMP_ALLOCATOR.lock().allocate(size, align).expect("out of memory")
}
#[no_mangle]
pub unsafe extern fn __rdl_oom(err: *const u8) -> ! {
   panic!("OOm");
}

#[no_mangle]
pub unsafe extern fn __rdl_dealloc(ptr: *mut u8,
                                   size: usize,
                                   align: usize) {
}
