RUSTC?=rustc
NASM?=nasm
LD?=ld

ASSEMBLIES=$(patsubst %.asm, %.o, $(wildcard *.asm))

all: kernel.bin

.PHONY: run
run: kernel.bin
	qemu-system-i386 -kernel $^

.PHONY: clean
clean:
	$(RM) kernel.bin *.o

$(ASSEMBLIES): %.o : %.asm
	$(NASM) -f elf32 -o $@ $<

main.o: main.rs
	$(RUSTC) -L. -O --target i386-unknown-linux --crate-type lib -o $@ --emit obj $^

kernel.bin: main.o $(ASSEMBLIES)
	$(LD) -m elf_i386 -T link.ld -o $@ $^
