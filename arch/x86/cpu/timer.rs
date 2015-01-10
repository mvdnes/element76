use platform::io;

static TIMER_COMMAND: u16 = 0x43;
static TIMER_CHANNEL0: u16 = 0x40;

pub fn set_interval(frequency: u32)
{
	let divisor = 1193180 / frequency;
	let l = divisor as u8;
	let h = (divisor >> 8) as u8;
	unsafe
	{
		io::outport(TIMER_COMMAND, 0x36); // 0x36 tells timer to repeat
		io::outport(TIMER_CHANNEL0, l);
		io::outport(TIMER_CHANNEL0, h);
	}
}
