/*
 * This file contains code for the Global Descriptor Table
 *
 * See: http://www.jamesmolloy.co.uk/tutorial_html/4.-The%20GDT%20and%20IDT.html
 */

static GDT_COUNT: uint = 5;
static mut gdt_entries: [GDTEntry,.. GDT_COUNT] = [GDTEntry { limit_low: 0, base_low: 0, base_middle: 0, access: 0, granularity: 0, base_high: 0 },.. GDT_COUNT];
static mut gdt_ptr: GDTPointer = GDTPointer { limit: 0, base: 0 };

#[repr(packed)]
struct GDTEntry
{
	limit_low: u16,
	base_low: u16,
	base_middle: u8,
	access: u8,
	granularity: u8,
	base_high: u8
}

#[repr(packed)]
struct GDTPointer
{
	limit: u16,
	base: u32
}

pub fn init_gdt()
{
	unsafe
	{
		gdt_ptr.limit = (::core::mem::size_of::<GDTEntry>() * GDT_COUNT - 1) as u16;
		gdt_ptr.base = &mut gdt_entries as *mut [GDTEntry,.. GDT_COUNT] as u32;

		gdt_set_gate(0, 0, 0, 0, 0);
		gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
		gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
		gdt_set_gate(3, 0, 0xFFFFFFFF, 0xFA, 0xCF);
		gdt_set_gate(4, 0, 0xFFFFFFFF, 0xF2, 0xCF);

		gdt_flush(&mut gdt_ptr as *mut GDTPointer as u32);
	};
}

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

extern
{
	fn gdt_flush(pointer: u32);
}
