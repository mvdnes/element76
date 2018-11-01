RUSTC?=rustc -g
CARGO?=cargo
NASM?=nasm
LD?=ld

ARCH_DEPENDENCIES=$(wildcard arch/x86/*/*.rs)
KERNEL_DEPENDENCIES=$(wildcard kernel/*.rs) $(wildcard kernel/*/*.rs)
RUST_DEPENDENCIES=$(ARCH_DEPENDENCIES) $(KERNEL_DEPENDENCIES)
ASSEMBLIES=$(patsubst %.asm, %.o, $(wildcard arch/x86/asm/*.asm))
TARGET=i686-unknown-linux-gnu
RUSTLIB=target/i686-unknown-linux-gnu/debug/libkernel.a
BINARY=bin/kernel.bin
RUSTC_OPTIONS=--target $(TARGET) -C panic=abort

all: $(BINARY)

.PHONY: run
run: $(BINARY)
	qemu-system-i386 -kernel $<

.PHONY: clean
clean:
	cargo clean
	$(RM) $(BINARY) *.o $(ASSEMBLIES)

$(ASSEMBLIES): %.o : %.asm
	$(NASM) -f elf32 -o $@ $<

$(RUSTLIB): kernel_x86.rs $(RUST_DEPENDENCIES)
	cargo build

$(BINARY): $(ASSEMBLIES) $(RUSTLIB)
	$(LD) -m elf_i386 -T link.ld -o $@ $^
