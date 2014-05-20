/*
 * main.rs
 */

#![no_std]
#![feature(globs)]
#![feature(asm)]

extern crate core;

use core::prelude::*;

fn write_screen(xpos: uint, ypos: uint, value: u8, mask: u8)
{
	if xpos >= 80 || ypos >= 25 { return }
	unsafe
	{
		*((0xb8000 + ypos*160 + xpos * 2) as *mut u8) = value;
		*((0xb8000 + ypos*160 + xpos * 2 + 1) as *mut u8) = mask;
	}
}

#[no_mangle]
#[no_split_stack]
pub fn main()
{
	for x in range(0u, 80)
	{
		for y in range(0u, 25)
		{
			write_screen(x, y, ' ' as u8, 12 << 4);
		}
	}

	let hw = "Hello, World!";
	for i in range(0u, hw.len())
	{
		write_screen(3+i, 3, hw[i],
		match hw[i]
		{
			0x61..0x7A => 11, // Lowercase ASCII
			_ => 14,
		});
	}
	halt();
}

fn halt()
{
	unsafe { asm!("hlt"); }
}
