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
