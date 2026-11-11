; Emit start/end/copy operations or dispatch a typed fetch from node RDI.
generate_vararg:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    cmp dword [rbx+N_OP], 2
    je .arg
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    cmp dword [rbx+N_OP], 3
    je .done
    cmp dword [rbx+N_OP], 4
    je .copy
    lea rdi, [i_rax_rcx]
    call emit
    mov r12, [rbx+N_SYM]
    lea rdi, [p_mov64]
    mov rsi, [r12+S_VA_GP]
    mov edx, 8
    call emit_immediate
    lea rdi, [i_store_32]
    call emit
    lea rdi, [p_mov64]
    mov rsi, [r12+S_VA_FP]
    mov edx, 8
    call emit_immediate
    lea rdi, [i_va_start_fp]
    call emit
    lea rdi, [p_incoming_addr]
    mov rsi, [r12+S_VA_STACK]
    add rsi, 16
    call emit_offset_load
    lea rdi, [i_va_start_overflow]
    call emit
    mov rdi, [r12+S_VA_AREA]
    call emit_frame_address
    lea rdi, [i_va_start_save]
    call emit
    jmp .done
.copy:
    call emit_push_value
    mov rdi, [rbx+N_RIGHT]
    call generate_expression
    call emit_pop_rcx
    mov edi, 24
    call emit_copy_count
    jmp .done
.arg:
    mov rdi, rbx
    call generate_va_arg
.done:
    add rsp, 8
    pop r12
    pop rbx
    ret
