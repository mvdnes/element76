use kernel::stdio::StdioWriter;
use platform::vga::{White, Black};

static mut tick: uint = 0;

pub fn handle_irq()
{
	let mut printer = StdioWriter { xpos: 0, ypos: 10, fg: White, bg: Black };
	let usetick = unsafe
	{
		tick = (tick + 1) % 50;
		tick >= 25
	};
	printer.print_screen(if usetick { "tick" } else { "tock" });
}
