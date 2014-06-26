RUSTC?=rustc
NASM?=nasm
LD?=ld

ARCH_DEPENDENCIES=$(wildcard arch/x86/*/*.rs)
KERNEL_DEPENDENCIES=$(wildcard kernel/*.rs)
KERNEL_DEPENDENCIES=$(wildcard kernel/*/*.rs)
RUST_DEPENDENCIES=$(ARCH_DEPENDENCIES) $(KERNEL_DEPENDENCIES)
ASSEMBLIES=$(patsubst %.asm, %.o, $(wildcard arch/x86/asm/*.asm))
TARGET=i686-unknown-linux-gnu

all: kernel.bin

.PHONY: run
run: kernel.bin
	qemu-system-i386 -kernel $<

.PHONY: clean
clean:
	$(RM) kernel.bin *.o $(ASSEMBLIES) main.a

$(ASSEMBLIES): %.o : %.asm
	$(NASM) -f elf32 -o $@ $<


main.a: main_x86.rs $(RUST_DEPENDENCIES)
	$(RUSTC) -O -L rustlibdir --target $(TARGET) $< -o $@

kernel.bin: $(ASSEMBLIES) main.a
	$(LD) --gc-sections -m elf_i386 -e _start -T link.ld -o $@ $^
