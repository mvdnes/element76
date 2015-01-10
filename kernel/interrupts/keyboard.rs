use kernel::stdio::StdioWriter;
use kernel::keyboard::*;
use platform::vga::Color;

static mut shift: u32 = 0;
static mut irqprinter: StdioWriter = StdioWriter{ xpos: 0, ypos: 4, fg: Color::Yellow, bg: Color::LightRed };

pub fn keyboard_irq()
{
	let mut printer = unsafe { irqprinter };
	match ::kernel::keyboard::get_key()
	{
		KeyboardAction::KeyUp(KeyboardKey::Escape) => { ::platform::cpu::request_int3(); },
		KeyboardAction::KeyUp(KeyboardKey::Shift) => unsafe { shift -= 1; },
		KeyboardAction::KeyDown(key) => match key
		{
			KeyboardKey::Printable(c, d) => { printer.print_char(if unsafe {shift == 0} {c} else {d}); },
			KeyboardKey::Backspace => { printer.backspace(); },
			KeyboardKey::Return => { printer.crlf(); },
			KeyboardKey::Shift => unsafe { shift += 1; },
			KeyboardKey::Tab => { printer.tab(); },
			KeyboardKey::Unknown(c) => { printer.print_hex(c as u32, 8); printer.print_char(' '); },
			_ => {},
		},
		_ => {},
	};
	unsafe { irqprinter = printer; };
}
