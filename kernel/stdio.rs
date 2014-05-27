use core::prelude::*;
use platform::vga::{Color, COLS, ROWS, Black, White};
use platform::vga;

pub struct StdioWriter
{
	pub xpos: uint,
	pub ypos: uint,
	pub fg: Color,
	pub bg: Color
}

impl StdioWriter
{
	pub fn new() -> StdioWriter
	{
		StdioWriter
		{
			xpos: 0,
			ypos: 0,
			fg: White,
			bg: Black
		}
	}

	#[no_split_stack]
	pub fn clear_screen(&mut self)
	{
		for x in range(0u, COLS)
		{
			for y in range(0u, ROWS)
			{
				vga::putc(x, y, 0);
				vga::setfg(x, y, self.fg);
				vga::setbg(x, y, self.bg);
			}
		}
		self.move(0, 0);
	}

	#[no_split_stack]
	pub fn move(&mut self, x: uint, y: uint)
	{
		self.move_coords(x, y);
		self.set_cursor();
	}

	#[no_split_stack]
	pub fn backspace(&mut self)
	{
		self.go_left();
		self.raw_print_char(' ' as u8);
		self.set_cursor();
	}

	#[no_split_stack]
	pub fn tab(&mut self)
	{
		let x = self.xpos;
		for _ in range(0, 4 - (x % 4))
		{
			self.raw_print_char(' ' as u8);
			self.go_right();
		}
		self.set_cursor();
	}

	#[no_split_stack]
	pub fn crlf(&mut self)
	{
		self.xpos = 0;
		self.ypos = if self.ypos == ROWS - 1 { 0 } else { self.ypos + 1 };
		self.set_cursor();
	}

	#[no_split_stack]
	fn go_right(&mut self)
	{
		if self.xpos == COLS - 1
		{
			self.xpos = 0;
			self.ypos = (self.ypos + ROWS + 1) % ROWS;
		}
		else
		{
			self.xpos += 1;
		}
	}

	#[no_split_stack]
	fn go_left(&mut self)
	{
		if self.xpos == 0
		{
			self.xpos = COLS - 1;
			self.ypos = (self.ypos + ROWS - 1) % ROWS;
		}
		else
		{
			self.xpos -= 1;
		}
	}

	#[no_split_stack]
	fn move_coords(&mut self, x: uint, y: uint)
	{
		let mut newx = x;
		let mut newy = y;
		if newx >= COLS { newx = 0; newy += 1; }
		if newy >= ROWS { newy = 0; }
		self.xpos = newx;
		self.ypos = newy;
	}

	#[no_split_stack]
	fn set_cursor(&self)
	{
		vga::move_cursor(self.xpos, self.ypos);
	}

	#[no_split_stack]
	pub fn print_bin(&mut self, v: u32)
	{
		self.print_screen("0b");

		for i in range(0, 32)
		{
			let c = match (v >> (31-i)) & 0x1
			{
				0 => '0',
				_ => '1',
			} as u8;
			self.raw_print_char(c);
			self.go_right();
		}
		self.set_cursor();
	}

	#[no_split_stack]
	pub fn print_hex(&mut self, v: u32)
	{
		self.print_screen("0x");

		for i in range(0, 8)
		{
			let c = match (v >> 4*(7-i)) & 0xF
			{
				c if c <= 9 => c + '0' as u32,
				c => c + -10 + 'A' as u32,
			} as u8;
			self.raw_print_char(c);
			self.go_right();
		}
		self.set_cursor();
	}

	#[no_split_stack]
	pub fn print_char(&mut self, value: char)
	{
		self.raw_print_char(value as u8);
		self.go_right();
		self.set_cursor();
	}

	#[no_split_stack]
	fn raw_print_char(&self, value: u8)
	{
		vga::putc(self.xpos, self.ypos, value as u8);
		vga::setfg(self.xpos, self.ypos, self.fg);
		vga::setbg(self.xpos, self.ypos, self.bg);
	}

	#[no_split_stack]
	pub fn print_screen(&mut self, value: &str)
	{
		for c in value.bytes()
		{
			self.raw_print_char(c);
			self.go_right();
		}
		self.set_cursor();
	}
}
