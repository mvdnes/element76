use core::prelude::*;
use platform::vga::{Color, COLS, ROWS};
use platform::vga;

#[no_split_stack]
pub fn clear_screen(color: Color)
{
	for x in range(0u, COLS)
	{
		for y in range(0u, ROWS)
		{
			vga::putc(x, y, 0);
			vga::setbg(x, y, color);
		}
	}
}

#[no_split_stack]
pub fn write_bin(xpos: uint, ypos: uint, value: u32, fg: Option<Color>, bg: Option<Color>) -> (uint, uint)
{
	let mut x = xpos;
	let mut y = ypos;
	let v = value;

	let (nx, ny) = write_screen(x, y, "0x", fg, bg);
	x = nx; y = ny;

	for i in range(0, 32)
	{
		let c = match (v >> (31-i)) & 0x1
		{
			0 => '0',
			_ => '1',
		};
		let (nx, ny) = write_char(x, y, c, fg, bg);
		x = nx; y = ny;
	}
	(x, y)
}

#[no_split_stack]
pub fn write_hex(xpos: uint, ypos: uint, value: u32, fg: Option<Color>, bg: Option<Color>) -> (uint, uint)
{
	let mut x = xpos;
	let mut y = ypos;
	let v = value;

	let (nx, ny) = write_screen(x, y, "0x", fg, bg);
	x = nx; y = ny;

	for i in range(0, 8)
	{
		let c = match (v >> 4*(7-i)) & 0xF
		{
			c if c <= 9 => (c + '0' as u32) as u8,
			c => (c + -10 + 'A' as u32) as u8,
		} as char;
		let (nx, ny) = write_char(x, y, c, fg, bg);
		x = nx; y = ny;
	}
	(x, y)
}

#[no_split_stack]
pub fn write_char(xpos: uint, ypos: uint, value: char, fg: Option<Color>, bg: Option<Color>) -> (uint, uint)
{
	let mut x = xpos;
	let mut y = ypos;
	if x >= COLS { x = 0; y += 1; }
	if y >= ROWS { y = 0; }

	vga::putc(x, y, value as u8);
	match fg
	{
		Some(color) => vga::setfg(x, y, color),
		None => {},
	};
	match bg
	{
		Some(color) => vga::setbg(x, y, color),
		None => {},
	};

	x += 1;
	if x >= COLS { x = 0; y += 1; }
	if y >= ROWS { y = 0; }
	vga::move_cursor(x, y);
	(x, y)
}

#[no_split_stack]
pub fn write_screen(xpos: uint, ypos: uint, value: &str, fg: Option<Color>, bg: Option<Color>) -> (uint, uint)
{
	let mut x = xpos;
	let mut y = ypos;
	for c in value.bytes()
	{
		let (nx, ny) = write_char(x, y, c as char, fg, bg);
		x = nx;
		y = ny;
	}
	(x, y)
}
