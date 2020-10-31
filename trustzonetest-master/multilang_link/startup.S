.type start, %function
.word start
.global start
start:
	ldr r1, =main
	bx r1
