.section .text
.globl _start
_start:
    lui a0, 2
    lui a1, 3
    add a0, a0, a1
    ret
