#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]
#![feature(asm, panic_implementation, panic_info_message)]

extern crate rlibc;

#[path = "arch/x86/"]
pub mod platform {
	pub mod vga;
	pub mod cpu;
	pub mod mmu;
	mod io;
	pub mod keyboard;
}

pub mod kernel {
	pub mod main;
	pub mod interrupts;
	mod stdio;
	mod keyboard;
}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
