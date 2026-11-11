; Fetch one promoted object, rolling both cursors back when either bank is full.
generate_va_arg:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    mov rbx, rdi
    mov r15, [rbx+N_TYPE]
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    lea rdi, [i_rax_r11]
    call emit
    xor edi, edi
    call label_new
    mov r13, rax
    xor edi, edi
    call label_new
    mov r14, rax
    mov rdi, r15
    call abi_shape
    mov r12d, eax
    mov qword [rsp], 0
    mov qword [rsp+8], 0
    mov ecx, 2
.count:
    mov edx, eax
    and edx, 255
    cmp edx, AC_X87
    jae .memory
    cmp edx, AC_INTEGER
    jne .count_fp
    add qword [rsp], 8
.count_fp:
    cmp edx, AC_SSE
    jne .count_next
    add qword [rsp+8], 16
.count_next:
    shr eax, 8
    dec ecx
    jnz .count
    lea rdi, [i_va_gp]
    call emit
    lea rdi, [i_va_fp]
    call emit
    lea rdi, [i_va_save]
    call emit
    cmp qword [rsp], 0
    je .fp_limit
    lea rdi, [p_va_gp_limit]
    mov esi, 48
    sub rsi, [rsp]
    mov edx, 4
    call emit_immediate
    lea rdi, [p_ja]
    mov rsi, r13
    call emit_branch
.fp_limit:
    cmp qword [rsp+8], 0
    je .advance
    lea rdi, [p_va_fp_limit]
    mov esi, 176
    sub rsi, [rsp+8]
    mov edx, 4
    call emit_immediate
    lea rdi, [p_ja]
    mov rsi, r13
    call emit_branch
.advance:
    lea rdi, [p_va_add_gp]
    mov rsi, [rsp]
    mov edx, 1
    call emit_immediate
    lea rdi, [p_va_add_fp]
    mov rsi, [rsp+8]
    mov edx, 1
    call emit_immediate
    cmp dword [r15+TY_KIND], K_STRUCT
    jae .aggregate
    lea rdi, [i_va_load_gp]
    cmp r12d, AC_INTEGER
    je .scalar
    lea rdi, [i_va_load_fp]
.scalar:
    call emit
    mov rdi, r15
    call emit_normalize
    jmp .register_done
.aggregate:
    mov qword [rsp], 0
.chunk:
    mov eax, r12d
    and eax, 255
    test eax, eax
    jz .next
    mov rdi, [rbx+N_TEMP]
    sub rdi, [rsp]
    call emit_frame_address
    lea rdi, [i_rax_rcx]
    call emit
    mov eax, r12d
    and eax, 255
    cmp eax, AC_INTEGER
    jne .fp_chunk
    lea rdi, [i_va_load_gp]
    call emit
    lea rdi, [i_va_next_gp]
    call emit
    jmp .store_chunk
.fp_chunk:
    lea rdi, [i_va_load_fp]
    call emit
    lea rdi, [i_va_next_fp]
    call emit
.store_chunk:
    lea rdi, [i_store_64]
    call emit
.next:
    shr r12d, 8
    add qword [rsp], 8
    cmp qword [rsp], 16
    jb .chunk
    mov rdi, [rbx+N_TEMP]
    call emit_frame_address
.register_done:
    lea rdi, [p_jump]
    mov rsi, r14
    call emit_branch
.memory:
    mov rdi, r13
    call label_define
    lea rdi, [i_va_overflow]
    call emit
    cmp dword [r15+TY_ALIGN], 8
    jbe .aligned
    lea rdi, [p_add_rax]
    mov esi, 15
    mov edx, 4
    call emit_immediate
    lea rdi, [p_va_and]
    mov esi, 0xfffffff0
    mov edx, 4
    call emit_immediate
.aligned:
    lea rdi, [p_va_after]
    mov esi, [r15+TY_BYTES]
    add rsi, 7
    and rsi, -8
    call emit_offset_load
    lea rdi, [i_va_overflow_store]
    call emit
    cmp dword [r15+TY_KIND], K_STRUCT
    jae .memory_aggregate
    mov rdi, r15
    call emit_load
    jmp .finish
.memory_aggregate:
    mov rdi, [rbx+N_TEMP]
    mov esi, [r15+TY_BYTES]
    call abi_copy_home
.finish:
    mov rdi, r14
    call label_define
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
