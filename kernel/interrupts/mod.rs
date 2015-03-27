use kernel::stdio::StdioWriter;
use platform::vga::Color;

mod timer;
mod keyboard;

pub fn handle_interrupt(interrupt_number: u32, error_code: u32)
{
	match interrupt_number
	{
		0x20 => timer::handle_irq(),
		0x21 => keyboard::keyboard_irq(),
		_ => unknown_irq(interrupt_number, error_code),
	};
}

fn unknown_irq(interrupt_number: u32, error_code: u32)
{
	let mut printer = StdioWriter::new();
	printer.fg = Color::White;
	printer.bg = Color::Black;
	printer.go_to(10, 5);
	printer.print_screen("Interrupt received");
	printer.fg = Color::Black;
	printer.bg = Color::White;
	printer.go_to(10, 6);
	printer.print_hex(interrupt_number, 32);
	printer.go_to(10, 7);
	printer.print_bin(error_code, 32);
}
