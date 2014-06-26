mod gdt;
mod idt;
mod pic;
mod timer;

static IRQ_OFFSET: u8 = 0x20;

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
	gdt::init_gdt();
	pic::remap_pic(IRQ_OFFSET);
	idt::init_idt();
	timer::set_interval(50);
	enable_interrupts();
}


#[no_mangle]
pub fn isr_handler(_ds: u32, _edi:u32, _esi:u32, _ebp:u32, _esp:u32, _ebx:u32, _edx:u32, _ecx:u32, _eax:u32, ino: u32, ec:u32, _eip:u32, _cs:u32, _eflags:u32, _useresp:u32, _ss:u32)
{
	::kernel::interrupts::handle_interrupt(ino, ec);

	// Ack IRQ
	if ino >= (IRQ_OFFSET as u32)
	{
		pic::acknowledge_irq(ino as u8 - IRQ_OFFSET);
	}
}

fn enable_interrupts()
{
	pic::enable_irq(0);
	pic::enable_irq(1);
	unsafe
	{
		asm!("sti");
	}
}
