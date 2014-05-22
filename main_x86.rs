#![no_std]
#![feature(globs)]
#![feature(asm)]

extern crate core;

#[path = "arch/x86/"]
mod platform {
	pub mod vga;
	pub mod cpu;
	mod io;
	pub mod keyboard;
}

pub mod kernel;
