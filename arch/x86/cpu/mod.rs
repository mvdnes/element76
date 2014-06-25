use platform::io;

pub static IRQ_OFFSET: uint = 0x20;

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

#[no_split_stack]
pub fn halt()
{
	unsafe { asm!("hlt"); };
}

#[no_split_stack]
pub fn request_int3()
{
	unsafe
	{
		asm!("int $$0x03");
	}
}

#[no_split_stack]
pub fn setup()
{
	init_gdt();
	remap_pic();
	init_idt();
	unsafe {
		io::outport(0x21,0xFD); // Keyboard interrupts only
		io::outport(0xA1,0xFF);
		asm!("sti");
	}
}

#[packed]
struct GDTEntry
{
	limit_low: u16,
	base_low: u16,
	base_middle: u8,
	access: u8,
	granularity: u8,
	base_high: u8
}

#[packed]
struct GDTPointer
{
	limit: u16,
	base: u32
}

#[packed]
struct IDTEntry
{
	base_low: u16,
	selector: u16,
	zero: u8,
	flags: u8,
	base_high: u16
}

#[packed]
struct IDTPointer
{
	limit: u16,
	base: u32
}

extern
{
	fn gdt_flush(pointer: u32);
	fn idt_flush(pointer: u32);
	fn isr0 ();
	fn isr1 ();
	fn isr2 ();
	fn isr3 ();
	fn isr4 ();
	fn isr5 ();
	fn isr6 ();
	fn isr7 ();
	fn isr8 ();
	fn isr9 ();
	fn isr10();
	fn isr11();
	fn isr12();
	fn isr13();
	fn isr14();
	fn isr15();
	fn isr16();
	fn isr17();
	fn isr18();
	fn isr19();
	fn isr20();
	fn isr21();
	fn isr22();
	fn isr23();
	fn isr24();
	fn isr25();
	fn isr26();
	fn isr27();
	fn isr28();
	fn isr29();
	fn isr30();
	fn isr31();
	fn irq0 ();
	fn irq1();
	fn irq2();
	fn irq3();
	fn irq4();
	fn irq5();
	fn irq6();
	fn irq7();
	fn irq8();
	fn irq9();
	fn irq10();
	fn irq11();
	fn irq12();
	fn irq13();
	fn irq14();
	fn irq15();
}

static GDT_COUNT: uint = 5;
static IDT_COUNT: uint = 256;
static mut gdt_entries: [GDTEntry,.. GDT_COUNT] = [GDTEntry { limit_low: 0, base_low: 0, base_middle: 0, access: 0, granularity: 0, base_high: 0 },.. GDT_COUNT];
static mut gdt_ptr: GDTPointer = GDTPointer { limit: 0, base: 0 };
static mut idt_entries: [IDTEntry,.. IDT_COUNT] = [IDTEntry { base_low: 0, selector: 0, zero: 0, flags: 0, base_high: 0 },.. IDT_COUNT];
static mut idt_ptr: IDTPointer = IDTPointer { limit: 0, base: 0 };

#[no_split_stack]
fn remap_pic()
{
	unsafe
	{
		let a1: u8 = io::inport(0x21);
		let a2: u8 = io::inport(0xA1);
		io::outport(0x20, 0x11);
		io::io_wait();
		io::outport(0xA0, 0x11);
		io::io_wait();
		io::outport(0x21, 0x20);                 // ICW2: Master PIC vector offset
		io::io_wait();
		io::outport(0xA1, 0x20+8);                 // ICW2: Slave PIC vector offset
		io::io_wait();
		io::outport(0x21, 4);                       // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
		io::io_wait();
		io::outport(0xA1, 2);                       // ICW3: tell Slave PIC its cascade identity (0000 0010)
		io::io_wait();

		io::outport(0x21, 0x01);
		io::io_wait();
		io::outport(0xA1, 0x01);
		io::io_wait();

		io::outport(0x21, a1);   // restore saved masks.
		io::outport(0xA1, a2);
	}
}

#[no_split_stack]
fn init_gdt()
{
	unsafe
	{
		gdt_ptr.limit = (::core::mem::size_of::<GDTEntry>() * GDT_COUNT - 1) as u16;
		gdt_ptr.base = &gdt_entries as *[GDTEntry,.. GDT_COUNT] as u32;

		gdt_set_gate(0, 0, 0, 0, 0);
		gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
		gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
		gdt_set_gate(3, 0, 0xFFFFFFFF, 0xFA, 0xCF);
		gdt_set_gate(4, 0, 0xFFFFFFFF, 0xF2, 0xCF);

		gdt_flush(&gdt_ptr as *GDTPointer as u32);
	};
}

#[no_split_stack]
unsafe fn gdt_set_gate(n: uint, base: u32, limit: u32, access: u8, gran: u8)
{
	gdt_entries[n].base_low = (base & 0xFFFF) as u16;
	gdt_entries[n].base_middle = ((base >> 16) & 0xFF) as u8;
	gdt_entries[n].base_high = ((base >> 24) & 0xFF) as u8;

	gdt_entries[n].limit_low = (limit & 0xFFFF) as u16;
	gdt_entries[n].granularity = ((limit >> 16) & 0x0F) as u8;

	gdt_entries[n].granularity |= gran & 0xF0;
	gdt_entries[n].access = access;
}

#[no_split_stack]
fn init_idt()
{
	unsafe
	{
		idt_ptr.limit = (::core::mem::size_of::<IDTEntry>() * IDT_COUNT - 1) as u16;
		idt_ptr.base = &idt_entries as *[IDTEntry,.. IDT_COUNT] as u32;

		idt_set_gate( 0, isr0  as u32, 0x08, 0x8E);
		idt_set_gate( 1, isr1  as u32, 0x08, 0x8E);
		idt_set_gate( 2, isr2  as u32, 0x08, 0x8E);
		idt_set_gate( 3, isr3  as u32, 0x08, 0x8E);
		idt_set_gate( 4, isr4  as u32, 0x08, 0x8E);
		idt_set_gate( 5, isr5  as u32, 0x08, 0x8E);
		idt_set_gate( 6, isr6  as u32, 0x08, 0x8E);
		idt_set_gate( 7, isr7  as u32, 0x08, 0x8E);
		idt_set_gate( 8, isr8  as u32, 0x08, 0x8E);
		idt_set_gate( 9, isr9  as u32, 0x08, 0x8E);
		idt_set_gate(10, isr10 as u32, 0x08, 0x8E);
		idt_set_gate(11, isr11 as u32, 0x08, 0x8E);
		idt_set_gate(12, isr12 as u32, 0x08, 0x8E);
		idt_set_gate(13, isr13 as u32, 0x08, 0x8E);
		idt_set_gate(14, isr14 as u32, 0x08, 0x8E);
		idt_set_gate(15, isr15 as u32, 0x08, 0x8E);
		idt_set_gate(16, isr16 as u32, 0x08, 0x8E);
		idt_set_gate(17, isr17 as u32, 0x08, 0x8E);
		idt_set_gate(18, isr18 as u32, 0x08, 0x8E);
		idt_set_gate(19, isr19 as u32, 0x08, 0x8E);
		idt_set_gate(20, isr20 as u32, 0x08, 0x8E);
		idt_set_gate(21, isr21 as u32, 0x08, 0x8E);
		idt_set_gate(22, isr22 as u32, 0x08, 0x8E);
		idt_set_gate(23, isr23 as u32, 0x08, 0x8E);
		idt_set_gate(24, isr24 as u32, 0x08, 0x8E);
		idt_set_gate(25, isr25 as u32, 0x08, 0x8E);
		idt_set_gate(26, isr26 as u32, 0x08, 0x8E);
		idt_set_gate(27, isr27 as u32, 0x08, 0x8E);
		idt_set_gate(28, isr28 as u32, 0x08, 0x8E);
		idt_set_gate(29, isr29 as u32, 0x08, 0x8E);
		idt_set_gate(30, isr30 as u32, 0x08, 0x8E);
		idt_set_gate(31, isr31 as u32, 0x08, 0x8E);
		idt_set_gate(32, irq0 as u32, 0x08, 0x8E);
		idt_set_gate(33, irq1 as u32, 0x08, 0x8E);
		idt_set_gate(34, irq2 as u32, 0x08, 0x8E);
		idt_set_gate(35, irq3 as u32, 0x08, 0x8E);
		idt_set_gate(36, irq4 as u32, 0x08, 0x8E);
		idt_set_gate(37, irq5 as u32, 0x08, 0x8E);
		idt_set_gate(38, irq6 as u32, 0x08, 0x8E);
		idt_set_gate(39, irq7 as u32, 0x08, 0x8E);
		idt_set_gate(40, irq8 as u32, 0x08, 0x8E);
		idt_set_gate(41, irq9 as u32, 0x08, 0x8E);
		idt_set_gate(42, irq10 as u32, 0x08, 0x8E);
		idt_set_gate(43, irq11 as u32, 0x08, 0x8E);
		idt_set_gate(44, irq12 as u32, 0x08, 0x8E);
		idt_set_gate(45, irq13 as u32, 0x08, 0x8E);
		idt_set_gate(46, irq14 as u32, 0x08, 0x8E);
		idt_set_gate(47, irq15 as u32, 0x08, 0x8E);

		idt_flush(&idt_ptr as *IDTPointer as u32);
	}
}

#[no_split_stack]
unsafe fn idt_set_gate(n: uint, base: u32, sel: u16, flags: u8)
{
	idt_entries[n].base_low = (base & 0xFFFF) as u16;
	idt_entries[n].base_high = ((base >> 16) & 0xFFFF) as u16;

	idt_entries[n].selector = sel;
	idt_entries[n].zero = 0;
	idt_entries[n].flags = (flags & 0b11100000) | 0b01110;
}

#[no_mangle]
#[no_split_stack]
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
	unsafe { io::outport(0x20, 0x20); };
	if ino >= 40
	{
		unsafe { io::outport(0xa0,0x20); };
	}
}
