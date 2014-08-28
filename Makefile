RUSTC?=rustc
NASM?=nasm
LD?=ld

ARCH_DEPENDENCIES=$(wildcard arch/x86/*/*.rs)
KERNEL_DEPENDENCIES=$(wildcard kernel/*.rs) $(wildcard kernel/*/*.rs)
RUST_DEPENDENCIES=$(ARCH_DEPENDENCIES) $(KERNEL_DEPENDENCIES)
ASSEMBLIES=$(patsubst %.asm, %.o, $(wildcard arch/x86/asm/*.asm))
TARGET=i686-unknown-linux-gnu
RUSTLIB=bin/main.a
BINARY=bin/kernel.bin

all: $(BINARY)

.PHONY: run
run: $(BINARY)
	qemu-system-i386 -kernel $<

.PHONY: clean
clean:
	$(RM) $(BINARY) *.o $(ASSEMBLIES) $(RUSTLIB)

$(ASSEMBLIES): %.o : %.asm
	$(NASM) -f elf32 -o $@ $<


$(RUSTLIB): main_x86.rs $(RUST_DEPENDENCIES)
	$(RUSTC) -L rustlibdir --target $(TARGET) $< -o $@

$(BINARY): $(ASSEMBLIES) $(RUSTLIB)
	$(LD) --gc-sections -m elf_i386 -T link.ld -o $@ $^
