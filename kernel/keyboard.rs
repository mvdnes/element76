use platform::keyboard;
use platform::keyboard::{ArchKeyUp, ArchKeyDown};

pub enum KeyboardKey
{
	Printable(char, char),
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
		2 => Printable('1', '!'),
		3 => Printable('2', '@'),
		4 => Printable('3', '#'),
		5 => Printable('4', '$'),
		6 => Printable('5', '%'),
		7 => Printable('6', '^'),
		8 => Printable('7', '&'),
		9 => Printable('8', '*'),
		10 => Printable('9', '('),
		11 => Printable('0', ')'),
		12 => Printable('-', '_'),
		13 => Printable('=', '+'),
		14 => Backspace,
		16 => Printable('q', 'Q'),
		17 => Printable('w', 'W'),
		18 => Printable('e', 'E'),
		19 => Printable('r', 'R'),
		20 => Printable('t', 'T'),
		21 => Printable('y', 'Y'),
		22 => Printable('u', 'U'),
		23 => Printable('i', 'I'),
		24 => Printable('o', 'O'),
		25 => Printable('p', 'P'),
		26 => Printable('[', '{'),
		27 => Printable(']', '}'),
		28 => Return,
		29 => Printable('\\', '|'),
		30 => Printable('a', 'A'),
		31 => Printable('s', 'S'),
		32 => Printable('d', 'D'),
		33 => Printable('f', 'F'),
		34 => Printable('g', 'G'),
		35 => Printable('h', 'H'),
		36 => Printable('j', 'J'),
		37 => Printable('k', 'K'),
		38 => Printable('l', 'L'),
		39 => Printable(';', ':'),
		40 => Printable('\'', '"'),
		42 => Shift,
		44 => Printable('z', 'Z'),
		45 => Printable('x', 'X'),
		46 => Printable('c', 'C'),
		47 => Printable('v', 'V'),
		48 => Printable('b', 'B'),
		49 => Printable('n', 'N'),
		50 => Printable('m', 'M'),
		51 => Printable(',', '<'),
		52 => Printable('.', '>'),
		53 => Printable('/', '?'),
		54 => Shift,
		57 => Space,
		c => Unknown(c),
	}
}
