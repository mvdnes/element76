pub unsafe fn outport<T>(address: u16, value: T)
{
	asm!("out $0, $1" :: "{ax}"(value), "N{dx}"(address));
}
