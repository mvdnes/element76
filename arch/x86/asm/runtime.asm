global __morestack

__morestack:
	cli
	hlt
	jmp __morestack
