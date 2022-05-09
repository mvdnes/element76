use ::core::arch::asm;

pub unsafe fn outport(address: u16, value: u8)
{
	asm!(
		"out dx, al",
		in("al") value,
		in("dx") address,
	);
}

pub unsafe fn inport(address: u16) -> u8
{
	let result;
	asm!(
		"in al, dx",
		in("dx") address,
		out("al") result,
	);
	result
}

pub fn io_wait()
{
	unsafe { outport(0x80, 0); };
}
