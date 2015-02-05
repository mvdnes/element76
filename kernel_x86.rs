#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]
#![feature(asm, lang_items)]
#![feature(core)]

extern crate core;
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

// Remove this when https://github.com/rust-lang/rust/issues/21827 is solved
mod std {
    pub mod option {
        pub use core::option::*;
    }
    pub mod iter {
        pub use core::iter::*;
    }
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
