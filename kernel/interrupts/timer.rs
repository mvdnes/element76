use crate::kernel::stdio::StdioWriter;
use crate::platform::vga::Color;

static mut TICK: u32 = 48;

pub fn handle_irq()
{
	let mut printer = StdioWriter { xpos: 0, ypos: 10, fg: Color::White, bg: Color::Black };
	let mytick = unsafe
	{
		TICK = (TICK + 1) % 50;
		TICK
	};
	if mytick % 25 == 0
	{
		printer.print_screen(if mytick < 25 { "tick" } else { "tock" });
	}
}
