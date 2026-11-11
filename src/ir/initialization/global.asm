; Emit constant initializer root RDI, padding every omitted region with zeros.
generate_global_initializer:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    mov qword [rsp], 0       ; Packed bytes not yet emitted.
    mov qword [rsp+8], 0     ; Number of pending bytes.
    mov r12, [rdi+N_TYPE]
    mov rbx, [rdi+N_ARGS]
    xor r13d, r13d
.leaf:
    test rbx, rbx
    jz .tail
    ; Keep the final shared byte pending until later fields are merged.
    ; Earlier bytes can be emitted once the next leaf begins after them.
.flush:
    mov rax, [rbx+N_VALUE]
    cmp rax, r13
    jb generation_error
    cmp qword [rsp+8], 0
    je .gap
    cmp rax, r13
    je .ready
    movzx esi, byte [rsp]
    shr qword [rsp], 8
    dec qword [rsp+8]
    inc r13
    mov edi, 1
    call emit_data_integer
    jmp .flush
.gap:
    mov rdi, [rbx+N_VALUE]
    sub rdi, r13
    call emit_data_zeros
    mov r13, [rbx+N_VALUE]
.ready:
    cmp qword [rbx+N_SYM], 0
    jne .constant
    cmp qword [rsp+8], 0
    jne generation_error
    cmp qword [rbx+N_AUX], 0
    jne .string
.constant:
    mov r15, [rbx+N_TYPE]
    cmp dword [r15+TY_KIND], K_STRUCT
    jae generation_error
    mov rdi, [rbx+N_LEFT]
    call evaluate_static_constant
    mov rdi, [rbx+N_LEFT]
    mov rdi, [rdi+N_TYPE]
    mov rsi, r15
    call static_convert
    test r8, r8
    jnz .address
    mov rdi, r15
    call constant_finite
    mov rdi, [rbx+N_SYM]
    test rdi, rdi
    jnz .bitfield
    mov r14, rdx
    mov rsi, rax
    mov edi, [r15+TY_BYTES]
    add r13, rdi
    cmp dword [r15+TY_KIND], K_LDOUBLE
    je .extended
    call emit_data_integer
    jmp .next
.address:
    cmp qword [rbx+N_SYM], 0
    jne generation_error
    mov rsi, rax
    mov rdi, r8
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    call emit_data_address
    add r13, 8
    jmp .next
.bitfield:
    mov ecx, [rdi+MEM_BITS]
    mov edx, 1
    shl rdx, cl
    dec rdx
    and rax, rdx
    mov ecx, [rdi+MEM_SHIFT]
    shl rax, cl
    or [rsp], rax
    add ecx, [rdi+MEM_BITS]
    add ecx, 7
    shr ecx, 3
    cmp rcx, [rsp+8]
    jbe .next
    mov [rsp+8], rcx
    jmp .next
.extended:
    mov edi, 8
    call emit_data_integer
    movzx esi, r14w
    mov edi, 8
    call emit_data_integer
    jmp .next
.string:
    mov rax, [rbx+N_LEFT]
    mov r14, [rax+N_VALUE]
    mov r15, [rbx+N_AUX]
    add r13, r15
.byte:
    movzx esi, byte [r14]
    mov edi, 1
    call emit_data_integer
    inc r14
    dec r15
    jnz .byte
.next:
    mov rbx, [rbx+N_NEXT]
    jmp .leaf
.tail:
    cmp qword [rsp+8], 0
    je .tail_zeros
    movzx esi, byte [rsp]
    shr qword [rsp], 8
    dec qword [rsp+8]
    inc r13
    mov edi, 1
    call emit_data_integer
    jmp .tail
.tail_zeros:
    mov edi, [r12+TY_BYTES]
    sub rdi, r13
    jc generation_error
    call emit_data_zeros
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
