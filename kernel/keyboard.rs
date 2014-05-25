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

#[no_split_stack]
pub fn get_key() -> KeyboardAction
{
	match keyboard::get_key()
	{
		ArchKeyDown(code) => KeyDown(parse_keycode(code)),
		ArchKeyUp(code) => KeyUp(parse_keycode(code)),
	}
}

#[no_split_stack]
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
		16 => Printable('q'),
		17 => Printable('w'),
		18 => Printable('e'),
		19 => Printable('r'),
		20 => Printable('t'),
		21 => Printable('y'),
		22 => Printable('u'),
		23 => Printable('i'),
		24 => Printable('o'),
		25 => Printable('p'),
		30 => Printable('a'),
		31 => Printable('s'),
		32 => Printable('d'),
		33 => Printable('f'),
		34 => Printable('g'),
		35 => Printable('h'),
		36 => Printable('j'),
		37 => Printable('k'),
		38 => Printable('l'),
		44 => Printable('z'),
		45 => Printable('x'),
		46 => Printable('c'),
		47 => Printable('v'),
		48 => Printable('b'),
		49 => Printable('n'),
		50 => Printable('m'),
		57 => Space,
		c => Unknown(c),
	}
}
