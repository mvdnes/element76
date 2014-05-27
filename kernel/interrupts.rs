use kernel::stdio::StdioWriter;
use kernel::keyboard;
use kernel::keyboard::*;
use platform::cpu::{Registers};
use platform::vga::{Black, White,Yellow, LightRed};

static mut shift: uint = 0;
static mut irqprinter: StdioWriter = StdioWriter{ xpos: 0, ypos: 4, fg: Yellow, bg: LightRed };

#[no_split_stack]
pub fn handle_interrupt(regs: Registers, interrupt_number: u32, error_code: u32) -> Registers
{
	match interrupt_number
	{
		1 => keyboard_irq(),
		_ => unknown_irq(interrupt_number, error_code),
	};
	regs
}

#[no_split_stack]
fn keyboard_irq()
{
	let mut printer = unsafe { irqprinter };
	match keyboard::get_key()
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

#[no_split_stack]
fn unknown_irq(interrupt_number: u32, error_code: u32)
{
	let mut printer = StdioWriter::new();
	printer.fg = White; printer.bg = Black;
	printer.move(10, 5);
	printer.print_screen("Interrupt received");
	printer.fg = Black; printer.bg = White;
	printer.move(10, 6);
	printer.print_hex(interrupt_number as uint, 32);
	printer.move(10, 7);
	printer.print_bin(error_code as uint, 32);
}
