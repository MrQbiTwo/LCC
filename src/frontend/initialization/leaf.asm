; Append one expression RCX at offset RSI, type RDI, to builder RDX.
initializer_leaf:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi
    mov r13, rsi
    mov r14, rdx
    mov r15, rcx
    mov edi, A_INIT
    mov rsi, [r15+N_TOKEN]
    call node_new
    mov [rax+N_TYPE], r12
    mov [rax+N_VALUE], r13
    mov [rax+N_LEFT], r15
    mov rbx, [r14+8]
    mov [rbx+N_NEXT], rax
    mov [r14+8], rax
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
