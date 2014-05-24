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
pub fn write_hex(xpos: uint, ypos: uint, value: uint, fg: Option<Color>, bg: Option<Color>) -> (uint, uint)
{
	let mut x = xpos;
	let mut y = ypos;
	let mut v = value;
	if v == 0
	{
		return write_char(xpos, ypos, '0', fg, bg);
	}
	while v != 0
	{
		let c = match v & 0xF
		{
			c if c <= 9 => (c +'0' as uint) as u8,
			c => (c + -10 + 'A' as uint) as u8,
		} as char;
		let (nx, ny) = write_char(xpos, ypos, c, fg, bg);
		x = nx; y = ny;
		v >>= 8;
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
