;;runtime.asm
use32

global start

extern main
start:
	call main
	hlt
