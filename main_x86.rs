#![no_std]
#![feature(globs)]
#![feature(asm)]

extern crate core;

#[path = "arch/x86/"]
mod platform {
	pub mod vga;
	pub mod cpu;
	pub mod io;
}

pub mod kernel;
