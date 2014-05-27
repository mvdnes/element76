use kernel::stdio::StdioWriter;
use kernel::keyboard::*;
use platform::vga::{Yellow, LightRed};

static mut shift: uint = 0;
static mut irqprinter: StdioWriter = StdioWriter{ xpos: 0, ypos: 4, fg: Yellow, bg: LightRed };

#[no_split_stack]
pub fn keyboard_irq()
{
	let mut printer = unsafe { irqprinter };
	match ::kernel::keyboard::get_key()
	{
		KeyUp(Escape) => { ::platform::cpu::request_irq3(); },
		KeyUp(Shift) => unsafe { shift -= 1; },
		KeyDown(key) => match key
		{
			Printable(c, d) => { printer.print_char(if unsafe {shift == 0} {c} else {d}); },
			Backspace => { printer.backspace(); },
			Return => { printer.crlf(); },
			Shift => unsafe { shift += 1; },
			Tab => { printer.tab(); },
			Unknown(c) => { printer.print_hex(c as uint, 8); printer.print_char(' '); },
			_ => {},
		},
		_ => {},
	};
	unsafe { irqprinter = printer; };
}
