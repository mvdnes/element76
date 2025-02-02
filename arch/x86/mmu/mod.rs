extern "C"
{
	static end: u32;
}

static mut PLACEMENT_ADDRESS: u32 = 0;

pub fn setup()
{
	unsafe
	{
		PLACEMENT_ADDRESS = end;
	}
}
