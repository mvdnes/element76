#![allow(ctypes)]
#![feature(globs)]
#![crate_type="staticlib"]
#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![crate_id = "main"]

extern crate core;

#[path = "arch/x86/"]
pub mod platform {
	pub mod vga;
	pub mod cpu;
	mod io;
	pub mod keyboard;
}

pub mod kernel {
	pub mod main;
	pub mod interrupts;
	mod stdio;
	mod keyboard;
}

#[lang = "begin_unwind"]
extern fn begin_unwind(_args: &core::fmt::Arguments, _file: &str, _line: uint) -> ! {
	loop {}
}
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
