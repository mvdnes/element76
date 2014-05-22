use platform::keyboard;
use platform::keyboard::{ArchKeyUp, ArchKeyDown};

pub enum KeyboardKey
{
	Printable(char),
	Space,
	Return,
	Backspace,
	Shift,
	Control,
	Alt,
	Escape,
	Unknown(u8)
}

pub enum KeyboardAction
{
	KeyUp(KeyboardKey),
	KeyDown(KeyboardKey),
}

pub fn get_key() -> KeyboardAction
{
	match keyboard::get_key()
	{
		ArchKeyDown(code) => KeyDown(parse_keycode(code)),
		ArchKeyUp(code) => KeyUp(parse_keycode(code)),
	}
}

fn parse_keycode(code: u8) -> KeyboardKey
{
	match code
	{
		1 => Escape,
		2 => Printable('1'),
		3 => Printable('2'),
		4 => Printable('3'),
		5 => Printable('4'),
		6 => Printable('5'),
		7 => Printable('6'),
		8 => Printable('7'),
		9 => Printable('8'),
		10 => Printable('9'),
		11 => Printable('0'),
		57 => Space,
		c => Unknown(c),
	}
}
