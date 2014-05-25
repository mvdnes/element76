use core::prelude::*;
use platform::vga::{Color, COLS, ROWS, Black, White};
use platform::vga;

static mut xpos: uint = 0;
static mut ypos: uint = 0;
static mut color_fg: Color = White;
static mut color_bg: Color = Black;

#[no_split_stack]
pub fn clear_screen()
{
	let color = unsafe { color_bg };
	for x in range(0u, COLS)
	{
		for y in range(0u, ROWS)
		{
			vga::putc(x, y, 0);
			vga::setbg(x, y, color);
		}
	}
	move(0, 0);
}

#[no_split_stack]
pub fn set_fg(color: Color)
{
	unsafe { color_fg = color; }
}

#[no_split_stack]
pub fn set_bg(color: Color)
{
	unsafe { color_bg = color; }
}

#[no_split_stack]
pub fn move(x: uint, y: uint)
{
	move_coords(x, y);
	set_cursor();
}

#[no_split_stack]
fn advance_coords()
{
	unsafe { move_coords(xpos + 1, ypos); };
}

#[no_split_stack]
fn move_coords(x: uint, y: uint)
{
	let mut newx = x;
	let mut newy = y;
	if newx >= COLS { newx = 0; newy += 1; }
	if newy >= ROWS { newy = 0; }
	unsafe
	{
		xpos = newx;
		ypos = newy;
	}
}

#[no_split_stack]
fn set_cursor()
{
	unsafe
	{
		vga::move_cursor(xpos, ypos);
	}
}

#[no_split_stack]
pub fn write_bin(x: uint, y: uint, v: u32)
{
	move_coords(x, y);
	print_bin(v);
}

#[no_split_stack]
pub fn print_bin(v: u32)
{
	print_screen("0b");

	for i in range(0, 32)
	{
		let c = match (v >> (31-i)) & 0x1
		{
			0 => '0',
			_ => '1',
		} as u8;
		raw_print_char(c);
		advance_coords();
	}
	set_cursor();
}

#[no_split_stack]
pub fn write_hex(x: uint, y: uint, v: u32)
{
	move_coords(x, y);
	print_hex(v);
}

#[no_split_stack]
pub fn print_hex(v: u32)
{
	print_screen("0x");

	for i in range(0, 8)
	{
		let c = match (v >> 4*(7-i)) & 0xF
		{
			c if c <= 9 => c + '0' as u32,
			c => c + -10 + 'A' as u32,
		} as u8;
		raw_print_char(c);
		advance_coords();
	}
	set_cursor();
}

#[no_split_stack]
pub fn write_char(x: uint, y: uint, value: char)
{
	move_coords(x, y);
	print_char(value);
}

#[no_split_stack]
pub fn print_char(value: char)
{
	raw_print_char(value as u8);
	advance_coords();
	set_cursor();
}

#[no_split_stack]
fn raw_print_char(value: u8)
{
	unsafe
	{
		vga::putc(xpos, ypos, value as u8);
		vga::setfg(xpos, ypos, color_fg);
		vga::setbg(xpos, ypos, color_bg);
	}
}

#[no_split_stack]
pub fn print_screen(value: &str)
{
	for c in value.bytes()
	{
		raw_print_char(c);
		advance_coords();
	}
	set_cursor();
}

#[no_split_stack]
pub fn write_screen(x: uint, y: uint, value: &str)
{
	move_coords(x, y);
	print_screen(value);
}
