pub mod entry;
pub mod active_page_table;

use memory::PAGE_SIZE;
use memory::Frame;
use memory::FrameAllocator;
use memory::table::*;

pub use self::entry::*;
pub use self::active_page_table::*;


pub const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 ||
                address >= 0xffff_8000_0000_0000,
                "invalid address: 0x{:x}", address);
        Page { number: address/ PAGE_SIZE }
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn p4_index(&self) -> usize {
    (self.number >> 27) & 0o777
}
    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }
    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }
    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

pub fn test_paging<A>(allocator: &mut A) where A:FrameAllocator {
    let mut page_table = unsafe { ActivePageTable::new() };

    let addr = 42 * 512 * 512 * 4096; // 42th P3 entry
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:?}, map to {:?}",
             page_table.translate(addr),
             frame);
    page_table.map_to(page, frame, EntryFlags::empty(), allocator);
    println!("Some = {:?}", page_table.translate(addr));
    println!("next free frame: {:?}", allocator.allocate_frame());

    page_table.unmap(Page::containing_address(addr), allocator);
    println!("None = {:?}", page_table.translate(addr));
}
