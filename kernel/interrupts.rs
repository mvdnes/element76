use core::prelude::*;
use platform::cpu::Registers;
use platform::vga::{write_screen, Black, White};

#[no_split_stack]
#[no_mangle]
pub fn isr_handler(_regs: Registers)
{
	write_screen(10, 10, 'x' as u8, Some(Black), Some(White));
}
