; Reconstruct an aggregate result in the call node RDI's padded local home.
generate_aggregate_receive:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov rdi, [rbx+N_TYPE]
    call abi_shape
    mov r12d, eax
    cmp eax, AC_MEMORY
    je .address
    cmp eax, AC_X87 | (AC_X87UP << 8)
    je .extended
    ; Save both GP results before any frame-address work can overwrite RAX.
    call emit_push_value
    lea rdi, [i_xmm0_rax]
    call emit
    call emit_push_value
    lea rdi, [i_xmm1_rax]
    call emit
    call emit_push_value
    xor r13d, r13d
    xor r14d, r14d
    xor r15d, r15d
.chunk:
    mov eax, r12d
    and eax, 255
    test eax, eax
    jz .next
    mov rdi, [rbx+N_TEMP]
    sub rdi, r13
    call emit_frame_address
    lea rdi, [i_rax_rcx]
    call emit
    mov eax, r12d
    and eax, 255
    cmp eax, AC_INTEGER
    je .integer
    mov esi, 16
    test r15, r15
    jz .sse_ready
    xor esi, esi
.sse_ready:
    inc r15
    jmp .load
.integer:
    lea rsi, [32+r14*8]
    inc r14
.load:
    lea rdi, [p_stack_load]
    call emit_offset_load
    lea rdi, [i_store_64]
    call emit
.next:
    shr r12d, 8
    add r13, 8
    cmp r13, 16
    jb .chunk
    lea rdi, [p_stack_add]
    mov esi, 48
    mov edx, 4
    call emit_immediate
    jmp .address
.extended:
    lea rdi, [i_ld_receive]
    call emit
    call emit_push_value
    mov rdi, [rbx+N_TEMP]
    call emit_frame_address
    lea rdi, [i_rax_rcx]
    call emit
    call emit_pop_value
    lea rdi, [type_ldouble]
    call emit_store
.address:
    mov rdi, [rbx+N_TEMP]
    call emit_frame_address
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
