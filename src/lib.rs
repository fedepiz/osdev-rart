#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(use_extern_macros)]
#![feature(alloc)]
#![feature(abi_x86_interrupt)]
#![feature(type_ascription)]
#![no_std]
extern crate rlibc;

#[macro_use]
extern crate bitflags;
extern crate cpuio;
extern crate multiboot2;
#[macro_use]
extern crate once;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate x86_64;

//Allocator
//extern crate bump_allocator;
extern crate hole_list_allocator;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate alloc;
#[macro_use]
mod macros;
mod vga_buffer;
mod memory;
mod interrupts;
mod port;
mod pic;
mod devices;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("Welcome to Rust-Art!");

    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };
    enable_nxe_bit();
    enable_write_protect_bit();

    memory::init(boot_info);

    use alloc::boxed::Box;
    {
        let heap_test = Box::new(42);
    }

    interrupts::init();
    println!("Initialization completed");
    unsafe {
        pic::initialize();
        devices::serial::initialize();
        devices::pit::initialize();
        x86_64::instructions::interrupts::enable();
    }
    logln!("Serial logging example {}", 42);

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
