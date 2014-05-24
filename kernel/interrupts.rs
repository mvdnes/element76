use core::prelude::*;
use kernel::stdio;
use platform::cpu::Registers;
use platform::vga::{Black, White};


#[no_split_stack]
#[no_mangle]
pub fn isr_handler(regs: Registers)
{
	stdio::write_screen(10, 10, "Interrupt received", Some(White), Some(Black));
	stdio::write_screen(10, 11, "0x", Some(Black), Some(White));
	stdio::write_hex(12, 11, regs.interrupt_number as uint, Some(Black), Some(White));
}
