use core::prelude::*;
use kernel::stdio;
use kernel::keyboard;
use kernel::keyboard::{KeyUp,KeyDown,Printable,Space,Escape};
use platform::cpu::{Registers};
use platform::vga::{Black, White,Yellow};

#[no_split_stack]
#[no_mangle]
pub fn handle_interrupt(regs: Registers, interrupt_number: u32, error_code: u32) -> Registers
{
	stdio::write_screen(10, 5, "Interrupt received", Some(White), Some(Black));
	stdio::write_hex(10, 6, interrupt_number as u32, Some(Black), Some(White));
	stdio::write_bin(10, 7, error_code as u32, Some(Black), Some(White));
	match interrupt_number
	{
		1 => keyboard_irq(),
		_ => {},
	};
	regs
}

fn keyboard_irq()
{
	match keyboard::get_key()
	{
		KeyUp(Escape) => { ::platform::cpu::request_irq3(); },
		KeyDown(key) => match key
		{
			Printable(c) => { stdio::write_char(1,1,c,Some(Yellow),None); },
			Space => { stdio::write_char(1,1,' ',Some(Yellow),None); },
			_ => {},
		},
		_ => {},
	};
}
