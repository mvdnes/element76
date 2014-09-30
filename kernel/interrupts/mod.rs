use kernel::stdio::StdioWriter;
use platform::vga::{Black, White};

mod timer;
mod keyboard;

#[no_split_stack]
pub fn handle_interrupt(interrupt_number: u32, error_code: u32)
{
	match interrupt_number
	{
		0x20 => timer::handle_irq(),
		0x21 => keyboard::keyboard_irq(),
		_ => unknown_irq(interrupt_number, error_code),
	};
}

#[no_split_stack]
fn unknown_irq(interrupt_number: u32, error_code: u32)
{
	let mut printer = StdioWriter::new();
	printer.fg = White; printer.bg = Black;
	printer.go_to(10, 5);
	printer.print_screen("Interrupt received");
	printer.fg = Black; printer.bg = White;
	printer.go_to(10, 6);
	printer.print_hex(interrupt_number as uint, 32);
	printer.go_to(10, 7);
	printer.print_bin(error_code as uint, 32);
}
