use kernel::stdio::StdioWriter;
use platform::vga::{White, Black};

static mut tick: uint = 48;

pub fn handle_irq()
{
	let mut printer = StdioWriter { xpos: 0, ypos: 10, fg: White, bg: Black };
	let mytick = unsafe
	{
		tick = (tick + 1) % 50;
		tick
	};
	if mytick % 25 == 0
	{
		printer.print_screen(if mytick < 25 { "tick" } else { "tock" });
	}
}
