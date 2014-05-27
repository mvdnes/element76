use kernel::stdio::StdioWriter;
use platform::cpu::Registers;
use platform::vga::{Black, White};

mod keyboard;

#[no_split_stack]
pub fn handle_interrupt(regs: Registers, interrupt_number: u32, error_code: u32) -> Registers
{
	match interrupt_number
	{
		1 => keyboard::keyboard_irq(),
		_ => unknown_irq(interrupt_number, error_code),
	};
	regs
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
