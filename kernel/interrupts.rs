use core::prelude::*;
use kernel::stdio;
use platform::cpu::Registers;
use platform::vga::{Black, White};


#[no_split_stack]
#[no_mangle]
pub fn isr_handler(regs: Registers)
{
	stdio::write_screen(10, 5, "Interrupt received", Some(White), Some(Black));
	stdio::write_hex(10, 6, regs.ds, Some(Black), Some(White));
	stdio::write_hex(10, 7, regs.edi, Some(Black), Some(White));
	stdio::write_hex(10, 8, regs.esi, Some(Black), Some(White));
	stdio::write_hex(10, 9, regs.ebp, Some(Black), Some(White));
	stdio::write_hex(10, 10, regs.esp, Some(Black), Some(White));
	stdio::write_hex(10, 11, regs.ebx, Some(Black), Some(White));
	stdio::write_hex(10, 12, regs.edx, Some(Black), Some(White));
	stdio::write_hex(10, 13, regs.ecx, Some(Black), Some(White));
	stdio::write_hex(10, 14, regs.eax, Some(Black), Some(White));
	stdio::write_hex(14, 15, regs.interrupt_number, Some(Black), Some(White));
	stdio::write_hex(14, 16, regs.error_code, Some(Black), Some(White));
	stdio::write_hex(10, 17, regs.eip, Some(Black), Some(White));
	stdio::write_hex(10, 18, regs.cs, Some(Black), Some(White));
	stdio::write_hex(10, 19, regs.eflags, Some(Black), Some(White));
	stdio::write_hex(10, 20, regs.useresp, Some(Black), Some(White));
	stdio::write_hex(10, 21, regs.ss, Some(Black), Some(White));
	::platform::cpu::ack_irs();
}
