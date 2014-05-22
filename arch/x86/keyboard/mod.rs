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
	KeyDown(KeyboardKey),
	KeyUp(KeyboardKey)
}

pub fn get_key() -> KeyboardAction
{
	let raw = unsafe { ::platform::io::inport(0x60) };
	let key = match raw & 0x7F
	{
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
		56 => Alt,
		code => Unknown(code),
	};
	match raw & 0x80
	{
		0 => KeyDown(key),
		_ => KeyUp(key),
	}
}
