RUSTC?=rustc
NASM?=nasm
LD?=ld

ARCH_DEPENDENCIES=$(wildcard arch/x86/*/*.rs)
KERNEL_DEPENDENCIES=$(wildcard kernel/*.rs)
RUST_DEPENDENCIES=$(ARCH_DEPENDENCIES) $(KERNEL_DEPENDENCIES)
ASSEMBLIES=$(patsubst %.asm, %.o, $(wildcard arch/x86/asm/*.asm))

all: kernel.bin

.PHONY: run
run: kernel.bin
	qemu-system-i386 -kernel $<

.PHONY: clean
clean:
	$(RM) kernel.bin *.o $(ASSEMBLIES)

$(ASSEMBLIES): %.o : %.asm
	$(NASM) -f elf32 -o $@ $<

main.o: main_x86.rs $(RUST_DEPENDENCIES)
	$(RUSTC) -L. -O --target i386-unknown-linux --crate-type lib -o $@ --emit obj $<

kernel.bin: main.o $(ASSEMBLIES)
	$(LD) -m elf_i386 -T link.ld -o $@ $^
