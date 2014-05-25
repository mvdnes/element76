use kernel::stdio;
use kernel::keyboard;
use kernel::keyboard::{KeyUp,KeyDown,Printable,Space,Escape};
use platform::cpu::{Registers};
use platform::vga::{Black, White,Yellow, LightRed};

#[no_split_stack]
#[no_mangle]
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
	stdio::set_fg(Yellow);
	stdio::set_bg(LightRed);
	match keyboard::get_key()
	{
		KeyUp(Escape) => { ::platform::cpu::request_irq3(); },
		KeyDown(key) => match key
		{
			Printable(c) => { stdio::print_char(c); },
			Space => { stdio::print_char(' '); },
			_ => {},
		},
		_ => {},
	};
}

fn unknown_irq(interrupt_number: u32, error_code: u32)
{
	stdio::set_fg(White); stdio::set_bg(Black);
	stdio::write_screen(10, 5, "Interrupt received");
	stdio::set_fg(Black); stdio::set_bg(White);
	stdio::write_hex(10, 6, interrupt_number as u32);
	stdio::write_bin(10, 7, error_code as u32);
}
