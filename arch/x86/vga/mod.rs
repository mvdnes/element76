use platform::io;

pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Pink = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightPink = 13,
    Yellow = 14,
    White = 15,
}

pub static ROWS: uint = 25;
pub static COLS: uint = 80;

pub fn putc(xpos: uint, ypos: uint, value: u8)
{
	if xpos >= COLS || ypos >= ROWS { return }
	unsafe
	{
		*((0xb8000 + ypos * COLS * 2 + xpos * 2) as *mut u8) = value;
	}
}

pub fn setfg(xpos: uint, ypos: uint, value: Color)
{
	if xpos >= COLS || ypos >= ROWS { return }
	unsafe
	{
		let ptr = (0xb8000 + ypos * COLS * 2 + xpos * 2 + 1) as *mut u8;
		*ptr = (*ptr & 0xF0) | (value as u8 & 0x0F);
	}
}

pub fn setbg(xpos: uint, ypos: uint, value: Color)
{
	if xpos >= COLS || ypos >= ROWS { return }
	unsafe
	{
		let ptr = (0xb8000 + ypos * COLS * 2 + xpos * 2 + 1) as *mut u8;
		*ptr = (*ptr & 0x0F) | ((value as u8 << 4) & 0x70);
	}
}

pub fn move_cursor(xpos: uint, ypos: uint)
{
	if xpos >= COLS || ypos >= COLS { return };
	let pos = ypos * COLS + xpos;
	unsafe
	{
		io::outport(0x3D4, 14);
		io::outport(0x3D5, (pos >> 8) as u8);
		io::outport(0x3D4, 15);
		io::outport(0x3D5, pos as u8);
	}
}
