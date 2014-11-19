use platform::keyboard;
use platform::keyboard::ArchKeyboardAction;

pub enum KeyboardKey
{
	Printable(char, char),
	Return,
	Backspace,
	Shift,
	Escape,
	Tab,
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
		ArchKeyboardAction::Down(code) => KeyboardAction::KeyDown(parse_keycode(code)),
		ArchKeyboardAction::Up(code) => KeyboardAction::KeyUp(parse_keycode(code)),
	}
}

fn parse_keycode(code: u8) -> KeyboardKey
{
	match code
	{
		1 => KeyboardKey::Escape,
		2 => KeyboardKey::Printable('1', '!'),
		3 => KeyboardKey::Printable('2', '@'),
		4 => KeyboardKey::Printable('3', '#'),
		5 => KeyboardKey::Printable('4', '$'),
		6 => KeyboardKey::Printable('5', '%'),
		7 => KeyboardKey::Printable('6', '^'),
		8 => KeyboardKey::Printable('7', '&'),
		9 => KeyboardKey::Printable('8', '*'),
		10 => KeyboardKey::Printable('9', '('),
		11 => KeyboardKey::Printable('0', ')'),
		12 => KeyboardKey::Printable('-', '_'),
		13 => KeyboardKey::Printable('=', '+'),
		14 => KeyboardKey::Backspace,
		15 => KeyboardKey::Tab,
		16 => KeyboardKey::Printable('q', 'Q'),
		17 => KeyboardKey::Printable('w', 'W'),
		18 => KeyboardKey::Printable('e', 'E'),
		19 => KeyboardKey::Printable('r', 'R'),
		20 => KeyboardKey::Printable('t', 'T'),
		21 => KeyboardKey::Printable('y', 'Y'),
		22 => KeyboardKey::Printable('u', 'U'),
		23 => KeyboardKey::Printable('i', 'I'),
		24 => KeyboardKey::Printable('o', 'O'),
		25 => KeyboardKey::Printable('p', 'P'),
		26 => KeyboardKey::Printable('[', '{'),
		27 => KeyboardKey::Printable(']', '}'),
		28 => KeyboardKey::Return,
		30 => KeyboardKey::Printable('a', 'A'),
		31 => KeyboardKey::Printable('s', 'S'),
		32 => KeyboardKey::Printable('d', 'D'),
		33 => KeyboardKey::Printable('f', 'F'),
		34 => KeyboardKey::Printable('g', 'G'),
		35 => KeyboardKey::Printable('h', 'H'),
		36 => KeyboardKey::Printable('j', 'J'),
		37 => KeyboardKey::Printable('k', 'K'),
		38 => KeyboardKey::Printable('l', 'L'),
		39 => KeyboardKey::Printable(';', ':'),
		40 => KeyboardKey::Printable('\'', '"'),
		41 => KeyboardKey::Printable('`', '~'),
		42 => KeyboardKey::Shift,
		43 => KeyboardKey::Printable('\\', '|'),
		44 => KeyboardKey::Printable('z', 'Z'),
		45 => KeyboardKey::Printable('x', 'X'),
		46 => KeyboardKey::Printable('c', 'C'),
		47 => KeyboardKey::Printable('v', 'V'),
		48 => KeyboardKey::Printable('b', 'B'),
		49 => KeyboardKey::Printable('n', 'N'),
		50 => KeyboardKey::Printable('m', 'M'),
		51 => KeyboardKey::Printable(',', '<'),
		52 => KeyboardKey::Printable('.', '>'),
		53 => KeyboardKey::Printable('/', '?'),
		54 => KeyboardKey::Shift,
		55 => KeyboardKey::Printable('*', '*'),
		57 => KeyboardKey::Printable(' ', ' '),
		71 => KeyboardKey::Printable('7', '7'),
		72 => KeyboardKey::Printable('8', '8'),
		73 => KeyboardKey::Printable('9', '9'),
		74 => KeyboardKey::Printable('-', '-'),
		75 => KeyboardKey::Printable('4', '4'),
		76 => KeyboardKey::Printable('5', '5'),
		77 => KeyboardKey::Printable('6', '6'),
		78 => KeyboardKey::Printable('+', '+'),
		79 => KeyboardKey::Printable('1', '1'),
		80 => KeyboardKey::Printable('2', '2'),
		81 => KeyboardKey::Printable('3', '3'),
		82 => KeyboardKey::Printable('0', '0'),
		83 => KeyboardKey::Printable('.', '.'),
		c => KeyboardKey::Unknown(c),
	}
}
