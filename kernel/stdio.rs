use crate::platform::vga::{Color, COLS, ROWS};
use crate::platform::vga;

pub struct StdioWriter
{
	pub xpos: u32,
	pub ypos: u32,
	pub fg: Color,
	pub bg: Color
}

impl Copy for StdioWriter {}
impl Clone for StdioWriter { fn clone(&self) -> Self { *self } }

impl StdioWriter
{
	pub fn new() -> StdioWriter
	{
		StdioWriter
		{
			xpos: 0,
			ypos: 0,
			fg: Color::White,
			bg: Color::Black
		}
	}

	pub fn clear_screen(&mut self)
	{
		for y in 0u32 .. ROWS
		{
			for x in 0u32 .. COLS
			{
				vga::putc(x, y, 0);
				vga::setfg(x, y, self.fg);
				vga::setbg(x, y, self.bg);
			}
		}
		self.go_to(0, 0);
	}

	pub fn go_to(&mut self, x: u32, y: u32)
	{
		self.move_coords(x, y);
		self.set_cursor();
	}

	pub fn backspace(&mut self)
	{
		self.go_left();
		self.raw_print_char(' ' as u8);
		self.set_cursor();
	}

	pub fn tab(&mut self)
	{
		let x = self.xpos;
		for _ in 0 .. 4 - (x % 4)
		{
			self.raw_print_char(' ' as u8);
			self.go_right();
		}
		self.set_cursor();
	}

	pub fn crlf(&mut self)
	{
		self.xpos = 0;
		self.ypos = if self.ypos == ROWS - 1 { 0 } else { self.ypos + 1 };
		self.set_cursor();
	}

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

	fn move_coords(&mut self, x: u32, y: u32)
	{
		let mut newx = x;
		let mut newy = y;
		if newx >= COLS { newx = 0; newy += 1; }
		if newy >= ROWS { newy = 0; }
		self.xpos = newx;
		self.ypos = newy;
	}

	fn set_cursor(&self)
	{
		vga::move_cursor(self.xpos, self.ypos);
	}

	pub fn print_bin(&mut self, v: u32, sz: u32)
	{
		self.print_screen("0b");

		let mut i = (sz - 1) as i32;
		while i >= 0
		{
			let c = match (v >> (i as u32)) & 0x1
			{
				0 => '0',
				_ => '1',
			} as u8;
			self.raw_print_char(c);
			self.go_right();
			i -= 1;
		}
		self.set_cursor();
	}

	pub fn print_hex(&mut self, v: u32, sz: u32)
	{
		self.print_screen("0x");

		let mut i = (sz - 4) as i32;
		while i >= 0
		{
			let c = match (v >> (i as u32)) & 0xF
			{
				c if c <= 9 => c + '0' as u32,
				c => c - 10 + 'A' as u32,
			} as u8;
			self.raw_print_char(c);
			self.go_right();
			i -= 4;
		}
		self.set_cursor();
	}

	pub fn print_char(&mut self, value: char)
	{
		self.raw_print_char(value as u8);
		self.go_right();
		self.set_cursor();
	}

	fn raw_print_char(&self, value: u8)
	{
		vga::putc(self.xpos, self.ypos, value);
		vga::setfg(self.xpos, self.ypos, self.fg);
		vga::setbg(self.xpos, self.ypos, self.bg);
	}

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

impl ::core::fmt::Write for StdioWriter
{
	fn write_str(&mut self, s: &str) -> ::core::fmt::Result
	{
		for b in s.bytes()
		{
			if b == b'\n' {
				self.xpos = 0;
				self.ypos = if self.ypos == ROWS - 1 { 0 } else { self.ypos + 1 };
			}
			else {
				self.raw_print_char(b);
				self.go_right();
			}
		}
		self.set_cursor();
		Ok(())
	}
}
