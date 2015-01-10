use core::marker::Copy;

pub enum ArchKeyboardAction
{
	Down(u8),
	Up(u8)
}

impl Copy for ArchKeyboardAction {}

pub fn get_key() -> ArchKeyboardAction
{
	let raw = unsafe { ::platform::io::inport(0x60) };
	let key = raw & 0x7F;
	match raw & 0x80
	{
		0 => ArchKeyboardAction::Down(key),
		_ => ArchKeyboardAction::Up(key),
	}
}
