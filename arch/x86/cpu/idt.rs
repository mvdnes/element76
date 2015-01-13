/*
 * This file contains code for the Interrupt Descriptor Table
 *
 * See: http://www.jamesmolloy.co.uk/tutorial_html/4.-The%20GDT%20and%20IDT.html
 */

use core::marker::Copy;

const IDT_COUNT: usize = 256;
static mut idt_entries: [IDTEntry; IDT_COUNT] = [IDTEntry { base_low: 0, selector: 0, zero: 0, flags: 0, base_high: 0 }; IDT_COUNT];
static mut idt_ptr: IDTPointer = IDTPointer { limit: 0, base: 0 };

#[repr(packed)]
struct IDTEntry
{
	base_low: u16,
	selector: u16,
	zero: u8,
	flags: u8,
	base_high: u16
}

impl Copy for IDTEntry {}

#[repr(packed)]
struct IDTPointer
{
	limit: u16,
	base: usize
}

pub fn init_idt()
{
	unsafe
	{
		idt_ptr.limit = (::core::mem::size_of::<IDTEntry>() * IDT_COUNT - 1) as u16;
		idt_ptr.base = &idt_entries as *const [IDTEntry; IDT_COUNT] as usize;

		idt_set_gate( 0, isr0  as usize, 0x08, 0x8E);
		idt_set_gate( 1, isr1  as usize, 0x08, 0x8E);
		idt_set_gate( 2, isr2  as usize, 0x08, 0x8E);
		idt_set_gate( 3, isr3  as usize, 0x08, 0x8E);
		idt_set_gate( 4, isr4  as usize, 0x08, 0x8E);
		idt_set_gate( 5, isr5  as usize, 0x08, 0x8E);
		idt_set_gate( 6, isr6  as usize, 0x08, 0x8E);
		idt_set_gate( 7, isr7  as usize, 0x08, 0x8E);
		idt_set_gate( 8, isr8  as usize, 0x08, 0x8E);
		idt_set_gate( 9, isr9  as usize, 0x08, 0x8E);
		idt_set_gate(10, isr10 as usize, 0x08, 0x8E);
		idt_set_gate(11, isr11 as usize, 0x08, 0x8E);
		idt_set_gate(12, isr12 as usize, 0x08, 0x8E);
		idt_set_gate(13, isr13 as usize, 0x08, 0x8E);
		idt_set_gate(14, isr14 as usize, 0x08, 0x8E);
		idt_set_gate(15, isr15 as usize, 0x08, 0x8E);
		idt_set_gate(16, isr16 as usize, 0x08, 0x8E);
		idt_set_gate(17, isr17 as usize, 0x08, 0x8E);
		idt_set_gate(18, isr18 as usize, 0x08, 0x8E);
		idt_set_gate(19, isr19 as usize, 0x08, 0x8E);
		idt_set_gate(20, isr20 as usize, 0x08, 0x8E);
		idt_set_gate(21, isr21 as usize, 0x08, 0x8E);
		idt_set_gate(22, isr22 as usize, 0x08, 0x8E);
		idt_set_gate(23, isr23 as usize, 0x08, 0x8E);
		idt_set_gate(24, isr24 as usize, 0x08, 0x8E);
		idt_set_gate(25, isr25 as usize, 0x08, 0x8E);
		idt_set_gate(26, isr26 as usize, 0x08, 0x8E);
		idt_set_gate(27, isr27 as usize, 0x08, 0x8E);
		idt_set_gate(28, isr28 as usize, 0x08, 0x8E);
		idt_set_gate(29, isr29 as usize, 0x08, 0x8E);
		idt_set_gate(30, isr30 as usize, 0x08, 0x8E);
		idt_set_gate(31, isr31 as usize, 0x08, 0x8E);
		idt_set_gate(32, irq0  as usize, 0x08, 0x8E);
		idt_set_gate(33, irq1  as usize, 0x08, 0x8E);
		idt_set_gate(34, irq2  as usize, 0x08, 0x8E);
		idt_set_gate(35, irq3  as usize, 0x08, 0x8E);
		idt_set_gate(36, irq4  as usize, 0x08, 0x8E);
		idt_set_gate(37, irq5  as usize, 0x08, 0x8E);
		idt_set_gate(38, irq6  as usize, 0x08, 0x8E);
		idt_set_gate(39, irq7  as usize, 0x08, 0x8E);
		idt_set_gate(40, irq8  as usize, 0x08, 0x8E);
		idt_set_gate(41, irq9  as usize, 0x08, 0x8E);
		idt_set_gate(42, irq10 as usize, 0x08, 0x8E);
		idt_set_gate(43, irq11 as usize, 0x08, 0x8E);
		idt_set_gate(44, irq12 as usize, 0x08, 0x8E);
		idt_set_gate(45, irq13 as usize, 0x08, 0x8E);
		idt_set_gate(46, irq14 as usize, 0x08, 0x8E);
		idt_set_gate(47, irq15 as usize, 0x08, 0x8E);

		idt_flush(&idt_ptr as *const IDTPointer as u32);
	}
}

unsafe fn idt_set_gate(n: usize, base: usize, sel: u16, flags: u8)
{
	idt_entries[n].base_low = (base & 0xFFFF) as u16;
	idt_entries[n].base_high = ((base >> 16) & 0xFFFF) as u16;

	idt_entries[n].selector = sel;
	idt_entries[n].zero = 0;
	idt_entries[n].flags = (flags & 0b11100000) | 0b01110;
}

extern
{
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
	fn irq1 ();
	fn irq2 ();
	fn irq3 ();
	fn irq4 ();
	fn irq5 ();
	fn irq6 ();
	fn irq7 ();
	fn irq8 ();
	fn irq9 ();
	fn irq10();
	fn irq11();
	fn irq12();
	fn irq13();
	fn irq14();
	fn irq15();
}
