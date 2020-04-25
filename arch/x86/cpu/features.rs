pub fn enable_sse() {
    unsafe {
        llvm_asm!("mov %cr0, %eax
              andw $$0xFFFB, %ax
              orw $$0x2, %ax
              mov %eax, %cr0
              mov %cr4, %eax
              orw $$(3 << 9), %ax
              mov %eax, %cr4"
              :
              :
              : "eax", "cr0", "cr4"
              : "volatile");
    }
}

