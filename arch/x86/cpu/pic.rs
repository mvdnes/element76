use platform::io;

static PIC1: u16 = 0x20;
static PIC1_DATA: u16 = 0x21;
static PIC2: u16 = 0xA0;
static PIC2_DATA: u16 = 0xA1;

static IRQ_ACK: u8 = 0x20;
static ICW1: u8 = 0x11;
static ICW4: u8 = 0x01;

pub fn remap_pic(offset: u8)
{
	unsafe
	{
		// Initialize
		io::outport(PIC1, ICW1);
		io::io_wait();
		io::outport(PIC2, ICW1);
		io::io_wait();

		// Set offset
		io::outport(PIC1_DATA, offset);
		io::io_wait();
		io::outport(PIC2_DATA, offset + 8);
		io::io_wait();

		// Connect Master to slave
		io::outport(PIC1_DATA, 4);
		io::io_wait();
		io::outport(PIC2_DATA, 2);
		io::io_wait();

		// Finalize
		io::outport(PIC1_DATA, ICW4);
		io::io_wait();
		io::outport(PIC2_DATA, ICW4);
		io::io_wait();

		// Disable all interrupts
		io::outport(PIC1_DATA, 0xFF);
		io::outport(PIC2_DATA, 0xFF);
	}
}

pub fn enable_irq(irq: u32)
{
	let (port, line) = if irq < 8
	{
		(PIC1_DATA, irq)
	}
	else
	{
		(PIC2_DATA, irq - 8)
	};

	unsafe
	{
		let value = io::inport(port) & !(1 << line);
		io::outport(port, value);
	}
}

pub fn disable_irq(irq: u32)
{
	let (port, line) = if irq < 8
	{
		(PIC1_DATA, irq)
	}
	else
	{
		(PIC2_DATA, irq - 8)
	};

	unsafe
	{
		let value = io::inport(port) | (1 << line);
		io::outport(port, value);
	}
}

pub fn acknowledge_irq(irq: u8)
{
	if irq >= 8
	{
		unsafe { io::outport(PIC2, IRQ_ACK); }
	}
	unsafe { io::outport(PIC1, IRQ_ACK); }
}
