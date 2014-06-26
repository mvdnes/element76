/*
 *  link.ld
 */
OUTPUT_FORMAT(elf32-i386)
ENTRY(start)
SECTIONS
 {
   . = 0x100000;
   .multiboot : { *(.multiboot) KEEP(*(.multiboot)) }
   .text : { *(.bootstrap_stack) *(.text) }
   .data : { *(.data) }
   .bss  : { *(.bss)  }
 }
