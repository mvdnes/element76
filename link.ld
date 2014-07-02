/*
 *  link.ld
 */
OUTPUT_FORMAT(elf32-i386)
ENTRY(start)
SECTIONS
{
	. = 0x100000;
	.text :
	{
		*(.multiboot)
		*(.bootstrap_stack)
		*(.text)
		. = ALIGN(4096);
		KEEP(*(.multiboot))
	}
	.data :
	{
		*(.data)
		. = ALIGN(4096);
	}
	.bss :
	{
		*(.bss)
		. = ALIGN(4096);
	}
	end = .;
}
