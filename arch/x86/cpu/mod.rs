mod gdt;
mod idt;
mod pic;
mod timer;

static IRQ_OFFSET: u8 = 0x20;

pub struct Registers
{
	pub ds: u32,
	pub edi: u32,
	pub esi: u32,
	pub ebp: u32,
	pub esp: u32,
	pub ebx: u32,
	pub edx: u32,
	pub ecx: u32,
	pub eax: u32,
	pub eip: u32,
	pub cs: u32,
	pub eflags: u32,
	pub useresp: u32,
	pub ss: u32
}

pub fn halt()
{
	unsafe { asm!("hlt"); };
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
pub fn isr_handler(ds: u32, edi:u32, esi:u32, ebp:u32, esp:u32, ebx:u32, edx:u32, ecx:u32, eax:u32, ino: u32, ec:u32, eip:u32, cs:u32, eflags:u32,useresp:u32,ss:u32)
{
	let regs = Registers
	{
		ds: ds,
		edi: edi,
		esi: esi,
		ebp: ebp,
		esp: esp,
		ebx: ebx,
		edx: edx,
		ecx: ecx,
		eax: eax,
		eip: eip,
		cs: cs,
		eflags: eflags,
		useresp: useresp,
		ss: ss
	};

	::kernel::interrupts::handle_interrupt(regs, ino, ec);

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
