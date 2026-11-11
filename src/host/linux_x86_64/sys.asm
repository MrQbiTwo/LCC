; Linux syscall adapter with partial-I/O and interrupted-I/O handling.
section .text
; Write all RDX bytes at RSI to descriptor RDI; return zero or negative errno.
host_write_all:
    test rdx, rdx
    jz .done
.loop:
    mov eax, 1
    syscall
    cmp rax, -4
    je .loop
    test rax, rax
    js .return
    jz .zero
    add rsi, rax
    sub rdx, rax
    jnz .loop
.done:
    xor eax, eax
.return:
    ret
.zero:
    mov rax, -5
    ret
