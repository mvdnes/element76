#![no_std]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

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
