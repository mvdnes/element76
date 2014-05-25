pub enum ArchKeyboardAction
{
	ArchKeyDown(u8),
	ArchKeyUp(u8)
}

#[no_split_stack]
pub fn get_key() -> ArchKeyboardAction
{
	let raw = unsafe { ::platform::io::inport(0x60) };
	let key = raw & 0x7F;
	match raw & 0x80
	{
		0 => ArchKeyDown(key),
		_ => ArchKeyUp(key),
	}
}
