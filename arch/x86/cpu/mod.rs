use core::marker::Copy;
use core::clone::Clone;

mod gdt;
mod idt;
mod pic;
mod timer;
mod features;

static IRQ_OFFSET: u8 = 0x20;

#[repr(C)]
pub struct InterruptArguments {
	_ds: u32, _edi: u32, _esi: u32, _ebp: u32, _esp: u32, _ebx: u32, _edx: u32, _ecx: u32, _eax: u32,
	interrupt_number: u32,
	error_code: u32,
	_eip: u32, _cs: u32, _eflags: u32, _useresp: u32, _ss: u32,
}

impl Copy for InterruptArguments {}
impl Clone for InterruptArguments { fn clone(&self) -> Self { *self } }

pub fn idle()
{
	unsafe
	{
		asm!("hlt");
	}
}

pub fn halt() -> !
{
	loop
	{
		unsafe
		{
			asm!("cli");
			asm!("hlt");
		}
	}
}

pub fn request_int3()
{
	unsafe
	{
		asm!("int $$0x03");
	}
	pic::disable_irq(0);
}

pub fn setup()
{
    features::enable_sse();
	gdt::init_gdt();
	pic::remap_pic(IRQ_OFFSET);
	idt::init_idt();
	timer::set_interval(50);
}

#[no_mangle]
pub extern "C" fn isr_handler(args: &InterruptArguments, _fpu_sse_data: [u8; 512])
{
	::kernel::interrupts::handle_interrupt(args.interrupt_number, args.error_code);

	// Ack IRQ
	if args.interrupt_number >= (IRQ_OFFSET as u32)
	{
		pic::acknowledge_irq(args.interrupt_number as u8 - IRQ_OFFSET);
	}
}

pub fn enable_interrupts()
{
	pic::enable_irq(0);
	pic::enable_irq(1);
	unsafe
	{
		asm!("sti");
	}
}
