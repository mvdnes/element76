#[no_split_stack]
pub unsafe fn outport(address: u16, value: u8)
{
	asm!("out %al, %dx" :: "{al}"(value), "{dx}"(address));
}

#[no_split_stack]
pub unsafe fn inport(address: u16) -> u8
{
	let mut result;
	asm!("in %dx, %al" : "={al}"(result) : "{dx}"(address));
	result
}

#[no_split_stack]
pub fn io_wait()
{
	unsafe { outport(0x80, 0); };
}
