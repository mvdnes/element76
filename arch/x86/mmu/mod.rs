extern
{
	static end: u32;
}

static mut placement_address: u32 = 0;

pub fn setup()
{
	unsafe
	{
		placement_address = end;
	}
}

pub fn kmalloc(size: u32, align: bool) -> u32
{
	let mut pa = unsafe { placement_address };
	if align && (pa & 0xFFFFF000) != 0
	{
		pa &= 0xFFFFF000;
		pa += 0x1000;
	}
	unsafe
	{
		placement_address += size;
	}
	return pa;
}
