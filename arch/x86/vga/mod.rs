use core::prelude::*;

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

pub fn write_screen(xpos: uint, ypos: uint, value: u8, fg: Option<Color>, bg: Option<Color>)
{
	if xpos >= 80 || ypos >= 25 { return }
	unsafe
	{
		let index = 0xb8000 + ypos*160 + xpos*2;
		*(index as *mut u8) = value;
		let ptr = (index + 1) as *mut u8;

		match fg {
			Some(color) => { *ptr &= 0xF0; *ptr |= color as u8; },
			None => {},
		}
		match bg {
			Some(color) => { *ptr &= 0x0F; *ptr |= color as u8 << 4; },
			None => {},
		}
	}
}
