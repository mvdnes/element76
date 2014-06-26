global __morestack

__morestack:
	cli
	hlt
	jmp __morestack

; Allocate a 16KiB stack
section .bootstrap_stack
align 4
stack_bottom:
times 16384 db 0
stack_top:

extern entry

; Entry point
section .text
global start
start:
	cli
	; Set up the stack
	mov esp, stack_top
	; Make everything play nice with segmented stacks - see __morestack below
	mov [gs:0x30], dword 0
	call entry
	jmp hang

global abort
abort:
	jmp hang

hang:
	cli
	hlt
	jmp hang
