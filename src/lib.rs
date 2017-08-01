#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(use_extern_macros)]
#![feature(alloc)]
#![no_std]
extern crate rlibc;

#[macro_use]
extern crate bitflags;
extern crate cpuio;
extern crate multiboot2;
extern crate spin;
extern crate volatile;
extern crate x86_64;

//Allocator
extern crate bump_allocator;
#[macro_use]
extern crate alloc;

#[macro_use]
mod vga_buffer;
mod memory;
mod pic;
#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    enable_nxe_bit();
    enable_write_protect_bit();
    vga_buffer::clear_screen();
    println!("Welcome to Rust-Art!");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, lentgh: 0x{:x}", area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");
    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
    .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
    .max().unwrap();
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    println!("kernel_start:0x{:x}, kernel_end:0x{:x}", kernel_start, kernel_end);
    println!("multiboot_start:0x{:x}, multiboot_end:0x{:x}", multiboot_start, multiboot_end);

    use::memory::FrameAllocator;
    use::memory::AreaFrameAllocator;

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());

    memory::remap_the_kernel(&mut frame_allocator, boot_info);
    unsafe {
        pic::initialize();
    }
    println!("Initialization completed");
    loop{}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("      {}", fmt);
    loop{}
}
