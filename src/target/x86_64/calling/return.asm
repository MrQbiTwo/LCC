; Marshal the aggregate address in generated RAX to ABI result locations.
generate_aggregate_return:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    call abi_shape
    mov r12d, eax
    cmp eax, AC_MEMORY
    je .memory
    cmp eax, AC_X87 | (AC_X87UP << 8)
    je .extended
    ; Copy before reading full eightbytes, including an object's short tail.
    lea rdi, [p_stack_sub]
    mov esi, 16
    mov edx, 4
    call emit_immediate
    lea rdi, [p_stack_addr_rcx]
    xor esi, esi
    call emit_offset_load
    mov edi, [rbx+TY_BYTES]
    call emit_copy_count
    xor r13d, r13d
    xor r14d, r14d
    xor r15d, r15d
.chunk:
    mov eax, r12d
    and eax, 255
    test eax, eax
    jz .next
    lea rdi, [p_stack_load]
    mov rsi, r13
    call emit_offset_load
    mov eax, r12d
    and eax, 255
    cmp eax, AC_INTEGER
    je .integer
    lea rax, [abi_to_fp]
    mov rdi, [rax+r15*8]
    inc r15
    jmp .place
.integer:
    lea rdi, [i_rax_r10]
    test r14, r14
    jz .integer_ready
    lea rdi, [i_rax_rdx]
.integer_ready:
    inc r14
.place:
    call emit
.next:
    add r13, 8
    shr r12d, 8
    cmp r13, 16
    jb .chunk
    lea rdi, [p_stack_add]
    mov esi, 16
    mov edx, 4
    call emit_immediate
    test r14, r14
    jz .done
    lea rdi, [i_r10_rax]
    call emit
    jmp .done
.extended:
    lea rdi, [type_ldouble]
    call emit_load
    lea rdi, [i_ld_return]
    call emit
    jmp .done
.memory:
    call emit_push_value
    mov rax, [current_function]
    mov rdi, [rax+S_RETURN]
    call emit_frame_address
    lea rdi, [type_ulong]
    call emit_load
    lea rdi, [i_rax_rcx]
    call emit
    call emit_pop_value
    mov edi, [rbx+TY_BYTES]
    call emit_copy_count
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
